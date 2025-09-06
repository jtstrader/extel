use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use xtl_core::{Metadata, XtlFunction, update_meta};

pub(crate) fn unit(_attr: TokenStream, function: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(function as XtlFunction);
    update_meta!(&mut function, |meta| {
        meta.category = Some(xtl_core::tag::Category::Unit);
    });

    quote! { #function }.into()
}

pub(crate) fn integration(_attr: TokenStream, function: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(function as XtlFunction);
    update_meta!(&mut function, |meta| {
        meta.category = Some(xtl_core::tag::Category::Integration);
    });

    quote! { #function }.into()
}

pub(crate) fn system(_attr: TokenStream, function: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(function as XtlFunction);
    update_meta!(&mut function, |meta| {
        meta.category = Some(xtl_core::tag::Category::System);
    });

    quote! { #function }.into()
}
