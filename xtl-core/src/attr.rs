//! Attribute parsing logic.

use proc_macro2::{Punct, Span};
use quote::{ToTokens, quote};
use syn::{Meta, MetaList, Path, parse::Parse};

use crate::Metadata;

#[derive(Debug)]
pub struct AttributeList(pub(crate) Vec<XtlAttribute>);

impl AttributeList {
    pub fn pop_metadata_attr(&mut self) -> Option<syn::Result<XtlAttribute>> {
        let xtl_metadata_idxs = self
            .0
            .iter()
            .enumerate()
            .filter(|(_, a)| a.is_xtl_metadata)
            .map(|(k, _)| k)
            .collect::<Vec<_>>();
        eprintln!("GOT METADATA IDXS: {:?}", xtl_metadata_idxs);

        match xtl_metadata_idxs.len() {
            0 => None,
            1 => Some(Ok(self.0.remove(xtl_metadata_idxs[0]))),
            _ => Some(Err(syn::Error::new(
                Span::call_site(),
                "multiple xtl metadata declarations",
            ))),
        }
    }
}

#[derive(Debug)]
pub struct XtlAttribute {
    pub name: String,
    pub is_xtl_metadata: bool,
    pub attr_meta: Meta,
}

impl From<Metadata> for XtlAttribute {
    fn from(value: Metadata) -> Self {
        syn::parse2(value.into_token_stream()).unwrap()
    }
}

impl Parse for AttributeList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut attrs = Vec::new();

        while !input.is_empty() {
            eprintln!("{:?}", attrs);
            match input.parse::<XtlAttribute>() {
                Ok(attr) => attrs.push(attr),
                Err(_) => return Ok(AttributeList(attrs)),
            }
        }

        Ok(AttributeList(attrs))
    }
}

impl quote::ToTokens for AttributeList {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let attrs = &self.0;
        tokens.extend(quote! { #(#attrs)* });
    }
}

impl Parse for XtlAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if !lookahead.peek(syn::Token![#]) {
            return Err(lookahead.error());
        }

        let attr; /* data within the brackets (i.e. #[attr]) */
        _ = input.parse::<syn::Token![#]>()?;
        _ = syn::bracketed!(attr in input);

        let meta = attr.parse::<Meta>()?;
        let meta_name = get_full_path_name_for_attr(&meta);
        let is_xtl_metadata = meta_name == crate::CORE_TEST_MACRO_NAME;

        Ok(XtlAttribute {
            name: meta_name,
            attr_meta: meta,
            is_xtl_metadata,
        })
    }
}

impl quote::ToTokens for XtlAttribute {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let pound = Punct::new('#', proc_macro2::Spacing::Alone);
        let meta = &self.attr_meta;
        tokens.extend(quote! { #pound [ #meta ] });
    }
}

fn get_full_path_name_for_attr(meta: &Meta) -> String {
    let path_string_builder = |path: &Path| {
        // If the path has a leading colon, the join call below naturally adds it
        // as a part of the iterator chaining as long as the root_prefix is Some(_).
        let root_prefix = path.leading_colon.map(|_| String::new());
        let segments = path.segments.iter().map(|s| s.ident.to_string());
        root_prefix
            .into_iter()
            .chain(segments)
            .collect::<Vec<_>>()
            .join("::")
    };

    let path = match meta {
        Meta::Path(path) => path,
        Meta::List(MetaList { path, .. }) => path,
        Meta::NameValue(nv) => &nv.path,
    };

    path_string_builder(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attribute_parse_no_root() {
        let src = r#"
            #[example(
                val1 = true,
                val2 = "data",
                val3 = 3,
                optional_val = None
            )]
        "#;

        let attr_res = syn::parse_str::<XtlAttribute>(src);
        assert!(
            attr_res.is_ok(),
            "failed with syn::Error: {}",
            attr_res.unwrap_err()
        );

        let attr = attr_res.unwrap();
        assert_eq!(attr.name, "example");
        assert!(!attr.is_xtl_metadata);
    }

    #[test]
    fn attribute_parse_with_root() {
        let src = r#"
            #[::example(
                val1 = true,
                val2 = "data",
                val3 = 3,
                optional_val = None
            )]
        "#;

        let attr_res = syn::parse_str::<XtlAttribute>(src);
        assert!(
            attr_res.is_ok(),
            "failed with syn::Error: {}",
            attr_res.unwrap_err()
        );

        let attr = attr_res.unwrap();
        assert_eq!(attr.name, "::example");
        assert!(!attr.is_xtl_metadata);
    }

    #[test]
    fn attribute_parse_multi_path_no_root() {
        let src = r#"
            #[example::macros::tag1(
                val1 = true,
                val2 = "data",
                val3 = 3,
                optional_val = None
            )]
        "#;

        let attr_res = syn::parse_str::<XtlAttribute>(src);
        assert!(
            attr_res.is_ok(),
            "failed with syn::Error: {}",
            attr_res.unwrap_err()
        );

        let attr = attr_res.unwrap();
        assert_eq!(attr.name, "example::macros::tag1");
        assert!(!attr.is_xtl_metadata);
    }

    #[test]
    fn attribute_parse_xtl_metadata() {
        let src = format!(
            r#"
            #[{}(
                val1 = true,
                val2 = "data",
                val3 = 3,
                optional_val = None
            )]
        "#,
            crate::CORE_TEST_MACRO_NAME
        );

        let attr_res = syn::parse_str::<XtlAttribute>(&src);
        assert!(
            attr_res.is_ok(),
            "failed with syn::Error: {}",
            attr_res.unwrap_err()
        );

        let attr = attr_res.unwrap();
        assert_eq!(attr.name, crate::CORE_TEST_MACRO_NAME);
        assert!(attr.is_xtl_metadata);
    }

    #[test]
    fn attribute_parse_multi_path_with_root() {
        let src = r#"
            #[::example::macros::tag1(
                val1 = true,
                val2 = "data",
                val3 = 3,
                optional_val = None
            )]
        "#;

        let attr_res = syn::parse_str::<XtlAttribute>(src);
        assert!(
            attr_res.is_ok(),
            "failed with syn::Error: {}",
            attr_res.unwrap_err()
        );

        let attr = attr_res.unwrap();
        assert_eq!(attr.name, "::example::macros::tag1");
        assert!(!attr.is_xtl_metadata);
    }

    #[test]
    fn attribute_to_tokens_no_root() {
        // Load attribute first.
        let src = r#"
            #[example(
                val1 = true,
                val2 = "data",
                val3 = 3,
                optional_val = None
            )]"#;
        let attr_res = syn::parse_str::<XtlAttribute>(src);
        assert!(
            attr_res.is_ok(),
            "failed with syn::Error: {}",
            attr_res.unwrap_err()
        );

        let attr = attr_res.unwrap();
        let tokens = attr.to_token_stream();

        assert_eq!(
            tokens.to_string(),
            r#"# [example (val1 = true , val2 = "data" , val3 = 3 , optional_val = None)]"#
        );
    }

    #[test]
    fn attribute_to_tokens_with_root() {
        // Load attribute first.
        let src = r#"
            #[::example(
                val1 = true,
                val2 = "data",
                val3 = 3,
                optional_val = None
            )]"#;
        let attr_res = syn::parse_str::<XtlAttribute>(src);
        assert!(
            attr_res.is_ok(),
            "failed with syn::Error: {}",
            attr_res.unwrap_err()
        );

        let attr = attr_res.unwrap();
        let tokens = attr.to_token_stream();

        assert_eq!(
            tokens.to_string(),
            r#"# [:: example (val1 = true , val2 = "data" , val3 = 3 , optional_val = None)]"#
        );
    }

    #[test]
    fn attribute_to_tokens_multi_path_no_root() {
        // Load attribute first.
        let src = r#"
            #[example::macros::tag1(
                val1 = true,
                val2 = "data",
                val3 = 3,
                optional_val = None
            )]"#;

        let attr_res = syn::parse_str::<XtlAttribute>(src);
        assert!(
            attr_res.is_ok(),
            "failed with syn::Error: {}",
            attr_res.unwrap_err()
        );

        let attr = attr_res.unwrap();
        let tokens = attr.to_token_stream();

        assert_eq!(
            tokens.to_string(),
            r#"# [example :: macros :: tag1 (val1 = true , val2 = "data" , val3 = 3 , optional_val = None)]"#
        );
    }

    #[test]
    fn attribute_to_tokens_multi_path_with_root() {
        // Load attribute first.
        let src = r#"
            #[::example::macros::tag1(
                val1 = true,
                val2 = "data",
                val3 = 3,
                optional_val = None
            )]"#;

        let attr_res = syn::parse_str::<XtlAttribute>(src);
        assert!(
            attr_res.is_ok(),
            "failed with syn::Error: {}",
            attr_res.unwrap_err()
        );

        let attr = attr_res.unwrap();
        let tokens = attr.to_token_stream();

        assert_eq!(
            tokens.to_string(),
            r#"# [:: example :: macros :: tag1 (val1 = true , val2 = "data" , val3 = 3 , optional_val = None)]"#
        );
    }

    #[test]
    fn attribute_to_tokens_xtl_metadata() {
        // Load attribute first.
        let src = format!(
            r#"
            #[{}(
                val1 = true,
                val2 = "data",
                val3 = 3,
                optional_val = None
            )]"#,
            crate::CORE_TEST_MACRO_NAME
        );

        let attr_res = syn::parse_str::<XtlAttribute>(&src);
        assert!(
            attr_res.is_ok(),
            "failed with syn::Error: {}",
            attr_res.unwrap_err()
        );

        let attr = attr_res.unwrap();
        let tokens = attr.to_token_stream();

        let macro_name = syn::parse_str::<Path>(crate::CORE_TEST_MACRO_NAME)
            .expect("core test macro name is a valid Rust path");
        let expected_tokens = quote! {
            #[#macro_name(val1 = true, val2 = "data", val3 = 3, optional_val = None)]
        };

        assert_eq!(tokens.to_string(), expected_tokens.to_string());
    }
}
