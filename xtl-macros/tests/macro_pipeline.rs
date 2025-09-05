use xtl_macros::*;

// #[xtl::__xtl_test_metadata__(
//     test = true,
//     category = "unit",
//     priority = "critical",
//     ignore = "Ignore reason here"
// )]
// #[test]
// fn compile_main() {}

#[test]
#[test_enable]
#[unit]
fn foo() {
    let x: usize = 2;
}
