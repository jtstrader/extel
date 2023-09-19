extern crate proc_macro;

use proc_macro::{Ident, TokenStream, TokenTree};

#[proc_macro_attribute]
/// Convert a *single argument function* into a parameterized function. The expected function
/// signature is a single argument function (can be any type) that returns a
/// [`TestStatus`](extel::TestStatus). If this attribute is attached to a function, the function's
/// implementation will be changed to return a `Vec<TestStatus>` instead, and will take no
/// parameters.
///
/// While technically possible, this macro is not intended to be used to run tests
/// manually. This macro is specifically for the purpose of helping the [test
/// initializer](extel::init_test_suite) prepare parameterized tests.
///
/// # Example
/// ```rust
/// use extel::TestStatus;
/// use extel_parameterized::parameters;
///
/// #[parameters(2, 4)]
/// fn less_than_3(x: i32) -> TestStatus {
///     match x < 3 {
///         true => TestStatus::Success,
///         false => TestStatus::Fail(format!("{} >= 3", x)),
///     }
/// }
///
/// assert_eq!(
///     less_than_3(),
///     vec![
///         TestStatus::Success,
///         TestStatus::Fail(String::from("4 >= 3"))
///     ]
/// );
/// ```
pub fn parameters(attr: TokenStream, function: TokenStream) -> TokenStream {
    let mut tokens: Vec<TokenTree> = function.clone().into_iter().collect();

    if let Err(e) = validate_parameters_spec(&tokens) {
        panic!("{}", e);
    }

    // Get function name and parameter(s)
    let (func_name, span) = (tokens[1].to_string(), tokens[1].span());
    tokens[1] = TokenTree::Ident(Ident::new(&format!("__{}", func_name), span));

    // Build test runner
    let test_runner = format!(
        "[{}].into_iter().map(|input| __{}(input)).collect::<Vec<TestStatus>>()",
        attr, func_name
    );

    // Create wrapper around the input stream
    let final_func = format!(
        "fn {}() -> Vec<TestStatus> {{ {} {} }}",
        func_name,
        tokens.into_iter().collect::<TokenStream>(),
        test_runner,
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
