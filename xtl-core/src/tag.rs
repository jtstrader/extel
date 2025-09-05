//! Types used for test tagging.

use darling::FromMeta;
use quote::{ToTokens, quote};

#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
pub enum Category {
    Unit,
    Integration,
    System,
}

impl ToTokens for Category {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match self {
            Category::Unit => quote! { "unit" },
            Category::Integration => quote! { "integration" },
            Category::System => quote! { "system" },
        });
    }
}

#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
pub enum Priority {
    Critical,
    High,
    Normal,
    Low,
    None,
}

impl ToTokens for Priority {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match self {
            Priority::Critical => quote! { "critical"  },
            Priority::High => quote! { "high"  },
            Priority::Normal => quote! { "medium"  },
            Priority::Low => quote! { "low" },
            Priority::None => quote! { "none" },
        });
    }
}
