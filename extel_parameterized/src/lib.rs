extern crate proc_macro;

use proc_macro::{Ident, TokenStream, TokenTree};

#[proc_macro_attribute]
pub fn parameters(attr: TokenStream, function: TokenStream) -> TokenStream {
    let mut tokens: Vec<TokenTree> = function.clone().into_iter().collect();

    let func_name_idx = validate_parameters_spec(&tokens).unwrap();

    // Get function name and parameter(s)
    let (func_name, span) = (
        tokens[func_name_idx].to_string(),
        tokens[func_name_idx].span(),
    );

    let attr_list = attr.to_string();
    let inner_func_name = format!("__{}", func_name);

    tokens[func_name_idx] = TokenTree::Ident(Ident::new(&inner_func_name, span));

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
        "{} {}() -> ExtelResult {{ {} {} }}",
        tokens[0..func_name_idx]
            .iter()
            .map(|token| token.to_string())
            .collect::<Vec<_>>()
            .join(" "),
        func_name,
        tokens.into_iter().collect::<TokenStream>(),
        test_runner_tokens,
    );

    final_func.parse().unwrap()
}

/// Validate that the macro is being applied only to function. Return the resulting index of the
/// function name.
fn validate_parameters_spec(tokens: &[TokenTree]) -> Result<usize, &str> {
    let mut i: usize = 0;
    while i < tokens.len() {
        // The only allowed starting idents are
        //  - fn
        //  - pub fn
        //  - pub(crate) fn
        //  - pub(super) fn

        match &tokens[i] {
            TokenTree::Ident(ident) => match ident.to_string().as_str() {
                "fn" => return Ok(i + 1),
                "pub" => {}
                _ => return Err("parameters can only be applied to functions!"),
            },
            TokenTree::Group(_) => {}
            _ => unreachable!(),
        };

        i += 1;
    }

    Err("reached end of token stream")
}
