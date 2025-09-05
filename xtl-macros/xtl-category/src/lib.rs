use xtl_core::{XtlFunction, Metadata, update_meta};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned};

/// Unit test.
#[proc_macro_attribute]
pub fn unit(attr: TokenStream, function: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(function as XtlFunction);
    update_meta!(function, |meta| {
        meta.category = Some(xtl_core::tag::Category::Unit);
    });

    quote! { #function }.into()
}

/// Unit test.
#[proc_macro_attribute]
pub fn integration(_attr: TokenStream, function: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(function as XtlFunction);
    update_meta!(function, |meta| {
        meta.category = Some(xtl_core::tag::Category::Integration);
    });

    quote! { #function }.into()
}

#[proc_macro_attribute]
pub fn __xtl_test_metadata(attr: TokenStream, function: TokenStream) -> TokenStream {
    eprintln!(
        "======================================= XTL TEST ======================================="
    );

    let metadata = parse_macro_input!(attr as Metadata);
    let function = parse_macro_input!(function as syn::ItemFn);

    eprintln!("{:#?}", metadata);

    quote_spanned! {function.span()=> #function }.into()
}
