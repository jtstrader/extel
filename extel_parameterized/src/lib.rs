//! ## Extel Parameterized - Writing Parameterized Tests in Rust
//! Extel Parameterized, or just *parameterized*, is a proc macro crate that serves to offer a
//! [`parameters`](macro@crate::parameters) macro for converting single argument functions into a valid function that
//! `Extel` can interpret.
//!
//! ```rust
//! use extel::prelude::*;
//! use extel_parameterized::parameters;
//!
//! fn single_test() -> ExtelResult {
//!     let mut my_cmd = cmd!("echo -n \"hello world\"");
//!     let output = my_cmd.output()?;
//!     let string_output = String::from_utf8(output.stdout)?;
//!
//!     extel_assert!(
//!         string_output == *"hello world",
//!         "expected 'hello world', got '{}'",
//!         string_output
//!     )
//! }
//!
//! #[parameters(1, 2, -2, 4)]
//! fn param_test(x: i32) -> ExtelResult {
//!     extel_assert!(x > 0, "{} <= 0", x)
//! }
//!
//! fn main() {
//!     init_test_suite!(ExtelDemo, single_test, param_test);
//!     ExtelDemo::run(TestConfig::default());
//! }
extern crate proc_macro;

use proc_macro::{Ident, TokenStream, TokenTree};

#[proc_macro_attribute]
pub fn parameters(attr: TokenStream, function: TokenStream) -> TokenStream {
    let mut tokens: Vec<TokenTree> = function.clone().into_iter().collect();

    let func_name_idx = match validate_parameters_spec(&tokens) {
        Ok(name) => name,
        Err(e) => panic!("{}", e),
    };

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
        "[{attr_list}]
            .into_iter()
            .map({inner_func_name})
            .collect::<Vec<extel::ExtelResult>>()"
    );

    // Create wrapper around the input stream
    let final_func = format!(
        "{} {}() -> Vec<ExtelResult> {{ {} {} }}",
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
fn validate_parameters_spec(tokens: &[TokenTree]) -> Result<usize, &'static str> {
    let mut i: usize = 0;
    while i < tokens.len() {
        // The only allowed starting idents are
        //  - fn
        //  - pub fn
        //  - pub(crate) fn
        //  - pub(super) fn

        if let TokenTree::Ident(ident) = &tokens[i] {
            match ident.to_string().as_str() {
                "fn" => return Ok(i + 1),
                "pub" => {}
                _ => return Err("#[parameters(...)] can only be applied to functions"),
            };
        };

        i += 1;
    }

    Err("reached end of token stream")
}
