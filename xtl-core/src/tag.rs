//! A set of "tags" to assign metadata to a test function.

use darling::FromMeta;
use quote::{ToTokens, quote};

/// A test category.
#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
pub enum Category {
    /// Tests that focus on individual components in isolation.
    Unit,
    /// Tests that focus on component interactions.
    Integration,
    /// Tests that focus on the entire system end-to-end.
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

/// Test priority levels.
///
/// The purpose of the test priority is to offer a way to categorize the importance of various tests
/// across a workspace that may not necessarily share functionality. Normally tests are grouped by module
/// and crate, but maybe you'd like to only run the highest priority tests in your CI? Doing this in Rust
/// without dependencies can be a pain, but with XTL it's as simple as adding a priority tag.
#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
pub enum Priority {
    /// Highest priority tests. Failures should block a release.
    Critical,
    /// High priority tests. Failures should be investigated immediately.
    High,
    /// The default test priority.
    Normal,
    /// Lower priority tests. Failures may not block a release or may be ignored.
    Low,
    /// Tests without a set priority.
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
