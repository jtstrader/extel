/// A collection of category macros to assign the type of test.
pub mod category {
    pub use extel_category_macros::*;
}

/// A testing macro that marks a function as an Extel test.
pub use extel_test_macros::test_enable as test;
