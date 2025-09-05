use xtl_core::{XtlFunction, Metadata, tag, update_meta};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned};

/// Xtl test.
#[proc_macro_attribute]
pub fn test_enable(attr: TokenStream, function: TokenStream) -> TokenStream {
    eprintln!("{:?}", attr);

    eprintln!("ATTEMPTING TO PARSE");
    let mut function = parse_macro_input!(function as XtlFunction);
    eprintln!("PARSE COMPLETE");

    eprintln!("ATTEMPTING TO UPDATE");
    update_meta!(function, |meta| {
        meta.test = Some(true);
    });

    eprintln!("UPDATE COMPLETE");

    let res: TokenStream = quote! { #function }.into();
    eprintln!("{}", res.to_string());
    res
}

#[proc_macro_attribute]
pub fn __xtl_test_metadata(attr: TokenStream, function: TokenStream) -> TokenStream {
    eprintln!(
        "======================================= XTL TEST ======================================="
    );

    eprintln!("attr: {:?}", attr);
    eprintln!("function: {:?}", function);

    let metadata = parse_macro_input!(attr as Metadata);
    let function = parse_macro_input!(function as syn::ItemFn);

    eprintln!("{:#?}", metadata);

    quote! { #function }.into()
}
