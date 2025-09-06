//! Core functionality for XTL, the eXtended Testing Library.
//!
//! This crate provides the foundational types and macros for parsing, processing,
//! and transforming Rust functions with XTL test metadata. It serves as the backbone
//! for proc macros that enhance test functions with categorization, prioritization,
//! and other metadata-driven features.
//!
//! # Key Components
//!
//! The main idea of XTL is to keep the majority of changes out of the proc macros and within the core
//! implementation. In this way our proc macros become more like a "glue" layer that can be used solely
//! to modify the metadata of a function. The actual usage of the metadata is left up to downstream test
//! runners.
//!
//! The most important components when working with XTL proc macros are:
//!
//! - [`XtlFunction`]: Represents a valid Rust function with associated attributes (e.g. `#[xtl::test]`)
//! - [`Metadata`]: Configuration struct containing test metadata like its category and priority
//! - [`update_meta!`]: Macro for safely updating metadata within procedural macros
//!
//! # Usage
//!
//! This crate is primarily intended for use within procedural macro implementations.
//! The typical workflow involves parsing a function with XTL attributes, extracting
//! and modifying the metadata, then regenerating the function with updated attributes.
//!
//! You can add new features by modifying the metadata struct and its [`ToTokens`](quote::ToTokens)
//! implementation. The parse impl is already handled via [`darling`](https://docs.rs/darling/latest/darling/).
//! The [`update_meta!`](crate::update_meta) macro will automatically handle updating the metadata and
//! modifying the [`XtlFunction`] for you.
//!
//! ```ignore
//! /***** in xtl-core *****/
//! pub struct Metadata {
//!     /* pre-existing fields */
//!     pub new_feature_metadata: Option<String>,
//! }
//!
//! /***** in xtl-macros *****/
//! use proc_macro::TokenStream;
//! use syn::parse_macro_input;
//! use quote::quote;
//!
//! use xtl_core::{XtlFunction, Metadata, update_meta, tag};
//!
//! #[proc_macro_attribute]
//! pub fn my_test_macro(attr: TokenStream, function: TokenStream) -> TokenStream {
//!     let mut function = parse_macro_input!(function as XtlFunction);
//!     
//!     update_meta!(&mut function, |meta| {
//!         meta.new_feature_metadata = Some("insert-data-here");
//!     });
//!     
//!     quote! { #function }.into()
//! }
//! ```

#![deny(missing_docs)]

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Parse, spanned::Spanned};

use crate::attr::{AttributeList, XtlAttribute};

pub mod attr;
pub mod tag;

/// Obtain a [`Metadata`] attribute from a given [`XtlFunction`], update the metadata
/// obtained from it, and then modify the provided function with the newly modified metadata.
///
/// # Usage
///
/// This macro should only be used in proc macro functions that return the [`proc_macro::TokenStream`](https://doc.rust-lang.org/proc_macro/struct.TokenStream.html)
/// type. This macro will return early if a parsing error occurs, which is emitted as tokens back
/// to the compiler.
///
/// ```ignore
/// use proc_macro::TokenStream;
/// use syn::parse_macro_input;
/// use quote::quote;
///
/// use xtl_core::{XtlFunction, Metadata, update_meta, tag};
///
/// #[proc_macro_attribute]
/// pub fn my_test_macro(attr: TokenStream, function: TokenStream) -> TokenStream {
///     let mut function = parse_macro_input!(function as XtlFunction);
///
///     // Updates the metadata. Note that this modifies
///     // the function in place so it can be directly
///     // written back out to the token stream.
///     update_meta!(&mut function, |meta| {
///         meta.test = Some(true);
///         meta.category = Some(tag::Category::Unit);
///     });
///
///     quote! { #function }.into()
/// }
/// ```
///
/// # Returns
///
/// If a parsing failure occurs, this macro short circuits and returns a compiler error.
#[macro_export]
macro_rules! update_meta {
    ($function:expr, $updater:expr) => {{
        // Constraints:
        //   - $function must be a mutable reference to an XtlFunction
        //   - $updater must be a function that takes a mutable reference to a Metadata
        let updater: &dyn Fn(&mut Metadata) = &$updater;
        let func: &mut XtlFunction = $function;
        let metadata_attr = func.pop_metadata_attr();
        let mut metadata = match metadata_attr {
            Some(attr) => match Metadata::try_from(attr) {
                Ok(meta) => meta,
                Err(e) => {
                    return e.into_compile_error().into();
                }
            },
            None => Metadata::default(),
        };

        updater(&mut metadata);
        func.push_metadata(metadata);
    }};
}

pub(crate) const CORE_TEST_MACRO_NAME: &str = "::xtl_macros::__xtl_test_metadata";

/// A function with a set of attributes.
///
/// It is assumed that at least one attribute is an xtl attribute.
#[derive(Debug)]
pub struct XtlFunction {
    metadata: AttributeList,
    func: syn::ItemFn,
}

impl XtlFunction {
    /// Pop the metadata attribute from the function.
    pub fn pop_metadata_attr(&mut self) -> Option<XtlAttribute> {
        self.metadata.pop_metadata_attr()
    }

    /// Push a metadata attribute to the function.
    ///
    /// The metadata attribute is put on the end of the attribute list
    /// to ensure that it will be triggered later in the macro expansion
    /// process.
    pub fn push_metadata(&mut self, metadata: Metadata) {
        self.metadata.0.push(metadata.into());
    }
}

impl Parse for XtlFunction {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let metadata = input.parse::<AttributeList>()?;
        let func = input.parse::<syn::ItemFn>()?;

        Ok(XtlFunction { metadata, func })
    }
}

impl quote::ToTokens for XtlFunction {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let metadata = &self.metadata;
        let func = &self.func;

        tokens.extend(quote! {
            #metadata
            #func
        });
    }
}

/// Metadata for XTL test functions.
#[derive(Default, Debug, FromMeta)]
#[darling(derive_syn_parse)]
pub struct Metadata {
    /// If `xtl::test` has been enabled.
    pub test: Option<bool>,

    /// `xtl::category` setting.
    pub category: Option<tag::Category>,

    /// `xtl::priority` setting.
    pub priority: Option<tag::Priority>,

    /// `xtl::ignore` setting and a reason if provided.
    pub ignore: Option<Option<String>>,
}

impl TryFrom<XtlAttribute> for Metadata {
    type Error = syn::Error;

    fn try_from(value: XtlAttribute) -> Result<Self, Self::Error> {
        if !value.is_xtl_metadata {
            return Err(syn::Error::new(
                value.span(),
                "attribute is not xtl metadata",
            ));
        }

        Metadata::from_meta(&value.attr_meta)
            .map_err(|e| syn::Error::new(value.attr_meta.span(), e))
    }
}

impl quote::ToTokens for Metadata {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let test = self.test.unwrap_or(false);
        let category = self.category.as_ref();
        let priority = self.priority.as_ref();
        let ignore = self.ignore.as_ref().map(|inner| inner.as_ref());

        let opt_category = match category {
            Some(opt) => quote! { ,category = #opt },
            None => quote!(),
        };

        let opt_priority = match priority {
            Some(opt) => quote! { ,priority = #opt },
            None => quote!(),
        };

        let opt_ignore = match ignore {
            Some(yes_ignore) => match yes_ignore {
                Some(data) => quote! { ,ignore = #data },
                None => quote! { ,ignore = "N/A" },
            },
            None => quote!(),
        };

        let path = syn::parse_str::<syn::Path>(CORE_TEST_MACRO_NAME).unwrap();
        let attr = quote! {
            #[#path(
                test = #test

                /*
                 * These optional message all have a comma at the start
                 * since they may not be needed at all.
                 */
                #opt_category
                #opt_priority
                #opt_ignore
            )]
        };

        tokens.extend(attr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_to_tokens_all_fields() {
        let expected = quote! {
            #[::xtl_macros::__xtl_test_metadata(
                test = true,
                category = "unit",
                priority = "high",
                ignore = "test is slow"
            )]
            fn foo() {}
        };

        let metadata = Metadata {
            test: Some(true),
            category: Some(tag::Category::Unit),
            priority: Some(tag::Priority::High),
            ignore: Some(Some("test is slow".to_string())),
        };

        let actual = quote! {
            #metadata
            fn foo() {}
        };

        assert_eq!(expected.to_string(), actual.to_string());
    }

    #[test]
    fn metadata_to_tokens_default_fields() {
        let expected = quote! {
            #[::xtl_macros::__xtl_test_metadata(test = false)]
            fn foo() {}
        };

        let metadata = Metadata::default();

        let actual = quote! {
            #metadata
            fn foo() {}
        };

        assert_eq!(expected.to_string(), actual.to_string());
    }

    #[test]
    fn metadata_to_tokens_ignore_no_reason() {
        let expected = quote! {
            #[::xtl_macros::__xtl_test_metadata(
                test = true,
                category = "unit",
                priority = "high",
                ignore = "N/A"
            )]
            fn foo() {}
        };

        let metadata = Metadata {
            test: Some(true),
            category: Some(tag::Category::Unit),
            priority: Some(tag::Priority::High),
            ignore: Some(None),
        };
        let actual = quote! {
            #metadata
            fn foo() {}
        };

        assert_eq!(expected.to_string(), actual.to_string());
    }
}
