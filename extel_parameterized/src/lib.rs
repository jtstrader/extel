extern crate proc_macro;

use proc_macro::{Ident, TokenStream, TokenTree};

#[proc_macro_attribute]
/// Convert a *single argument function* into a parameterized function. The expected function
/// signature is a single argument function (can be any type) that returns an
/// [`ExtelResult`](extel::ExtelResult). If this attribute is attached to a function, the function's
/// implementation will be changed to return a `Vec<ExtelResult>` instead, and will take no
/// parameters.
///
/// While technically possible, this macro is not intended to be used to run tests
/// manually. This macro is specifically for the purpose of helping the [test
/// initializer](extel::init_test_suite) prepare parameterized tests.
///
/// # Example
/// ```rust
/// use extel::{fail, pass, ExtelResult, TestResultType, TestStatus};
/// use extel_parameterized::parameters;
///
/// #[parameters(2, 4)]
/// fn less_than_3(x: i32) -> ExtelResult {
///     match x < 3 {
///         true => pass!(),
///         false => fail!("{} >= 3", x),
///     }
/// }
///
/// assert_eq!(
///     less_than_3().get_test_result(),
///     TestResultType::Parameterized(vec![
///         TestStatus::Success,
///         TestStatus::Fail(String::from("4 >= 3"))
///     ])
/// );
/// ```
pub fn parameters(attr: TokenStream, function: TokenStream) -> TokenStream {
    let mut tokens: Vec<TokenTree> = function.clone().into_iter().collect();

    if let Err(e) = validate_parameters_spec(&tokens) {
        panic!("{}", e);
    }

    // Get function name and parameter(s)
    let (func_name, span) = (tokens[1].to_string(), tokens[1].span());
    let attr_list = attr.to_string();
    let inner_func_name = format!("__{}", func_name);

    tokens[1] = TokenTree::Ident(Ident::new(&inner_func_name, span));

    // Build test runner
    let test_runner_tokens = format!(
        "let extel_single_results = [{attr_list}].into_iter().map({inner_func_name}).collect::<Vec<extel::ExtelResult>>();\
        Box::new(
            extel_single_results
                .into_iter()
                .map(TryInto::<extel::TestStatus>::try_into)
                .flatten()
                .collect::<Vec<_>>(),
        )"

    );

    // Create wrapper around the input stream
    let final_func = format!(
        "fn {}() -> ExtelResult {{ {} {} }}",
        func_name,
        tokens.into_iter().collect::<TokenStream>(),
        test_runner_tokens,
    );

    final_func.parse().unwrap()
}

fn validate_parameters_spec(tokens: &[TokenTree]) -> Result<(), String> {
    // First, can only run on functions
    match &tokens[0] {
        TokenTree::Ident(ident) => {
            if ident.to_string() != *"fn" {
                return Err("parameters can only be applied to functions!".to_string());
            }
        }
        _ => unreachable!(),
    };

    Ok(())
}
