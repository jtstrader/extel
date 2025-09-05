/// A collection of category macros to assign the type of test.
///
/// A category is used to determine the fundamental purpose of the test. Is this testing a single
/// unit of code? Connections between services? The entire system? Given a category, xtl can offer
/// more granular control over what tests you want to run and what you may want to avoid.
///
/// ## Usage
///
///
/// ```
/// #[xtl::category::integration]
/// fn test_api_auth() {
///     /* test code here */
/// }
/// ```
pub mod category {
    #[doc(inline)]
    pub use xtl_macros::{integration, system, unit};
}

#[doc(inline)]
pub use xtl_macros::test_enable as test;
