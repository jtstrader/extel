#[test] // necessary to be compiled during a test session
#[xtl_macros::test_enable]
fn test_enable_compiles() {}

#[test]
#[xtl_macros::__xtl_test_metadata(test = true, category = "unit", priority = "critical")]
fn private_metadata_attribute_compiles() {}
