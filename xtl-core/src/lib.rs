use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Parse, spanned::Spanned};

use crate::attr::{AttributeList, XtlAttribute};

pub mod attr;
pub mod tag;

#[macro_export]
macro_rules! update_meta {
    ($function:ident, $updater:expr) => {{
        // Type constraints enforce that $updater some function that can update metadata.
        let updater: &dyn Fn(&mut Metadata) = &$updater;
        let metadata_attr = $function.pop_metadata_attr();
        let mut metadata = match metadata_attr {
            Some(res) => match res {
                Ok(data) => match Metadata::try_from(data) {
                    Ok(meta) => meta,
                    Err(e) => {
                        return e.into_compile_error().into();
                    }
                },
                Err(e) => {
                    return e.into_compile_error().into();
                }
            },
            None => Metadata::default(),
        };
        eprintln!("SUCCESSFULLY GOT METADATA OBJ: {:#?}", metadata);
        eprintln!("APPLYING UPDATER");
        updater(&mut metadata);
        eprintln!("INTERNAL UPDATE SUCCESSFUL");
        $function.push_metadata(metadata);
    }};
}

pub(crate) const CORE_TEST_MACRO_NAME: &str = "::xtl_test_macros::__xtl_test_metadata";

#[derive(Debug)]
pub struct XtlFunction {
    pub metadata: AttributeList,
    pub func: syn::ItemFn,
}

impl XtlFunction {
    pub fn pop_metadata_attr(&mut self) -> Option<syn::Result<XtlAttribute>> {
        self.metadata.pop_metadata_attr()
    }

    pub fn push_metadata(&mut self, metadata: Metadata) {
        eprintln!("PUSHING METADATA: {:#?}", metadata);
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
            #[::xtl_test_macros::__xtl_test_metadata(
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
            #[::xtl_test_macros::__xtl_test_metadata(test = false)]
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
            #[::xtl_test_macros::__xtl_test_metadata(
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
