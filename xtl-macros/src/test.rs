use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use xtl_core::{Metadata, XtlFunction, update_meta};

/// Mark as an xtl test.
pub(crate) fn test_enable(_attr: TokenStream, function: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(function as XtlFunction);
    update_meta!(&mut function, |meta| {
        meta.test = Some(true);
    });
    quote! { #function }.into()
}

/// Process the xtl metadata for the test runner.
pub(crate) fn __xtl_test_metadata(attr: TokenStream, function: TokenStream) -> TokenStream {
    let _metadata = parse_macro_input!(attr as Metadata);
    let function = parse_macro_input!(function as syn::ItemFn);

    /* do something with metadata here */

    quote! { #function }.into()
}
