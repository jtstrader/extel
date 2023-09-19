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
    tokens[1] = TokenTree::Ident(Ident::new(&format!("__{}", func_name), span));

    // Build test runner
    let test_runner = format!(
        "[{}].into_iter().map(|input| __{}(input)).collect::<Vec<TestStatus>>()",
        attr,
        func_name
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
