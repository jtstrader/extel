/// A collection of category macros to assign the type of test.
pub mod category {
    pub use xtl_category_macros::*;
}

/// A testing macro that marks a function as an xtl test.
pub use xtl_test_macros::test_enable as test;
