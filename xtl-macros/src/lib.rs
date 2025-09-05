//! The official set of proc macros used by xtl to generate test metadata.

#![deny(missing_docs)]

use proc_macro::TokenStream;

macro_rules! xtl_attribute {
    ($(#[$attr:meta])* $func:path => $name:ident) => {
        $(#[$attr])*
        #[proc_macro_attribute]
        pub fn $name(attr: TokenStream, function: TokenStream) -> TokenStream {
            $func(attr, function)
        }
    };
}

mod category;
mod test;

xtl_attribute! {
    #[doc(hidden)]
    crate::test::__xtl_test_metadata => __xtl_test_metadata
}

xtl_attribute! {
    /// Register a function as an xtl test.
    crate::test::test_enable => test_enable
}

xtl_attribute! {
    /// Register an xtl test as a unit test.
    crate::category::unit => unit
}

xtl_attribute! {
    /// Register an xtl test as an integration test.
    crate::category::integration => integration
}

xtl_attribute! {
    /// Register an xtl test as a system/end-to-end test.
    crate::category::system => system
}
