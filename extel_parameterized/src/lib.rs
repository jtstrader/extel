extern crate proc_macro;

use proc_macro::{Ident, TokenStream, TokenTree};

#[proc_macro_attribute]
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
