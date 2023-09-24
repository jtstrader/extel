# Extel - Extended Testing Library
Extel is a testing library for Rust that supports both unit/integration testing for Rust projects and external APIs
through FFI or std::process.

# Using Extel
Extel is designed to work with *stateless functions*. Originally built to test executables built in C, Extel follows the
philosphy of both unit and integration tests not requiring any internal state. If any parameters or arguments are needed
specifically for testing external APIs, these should be reflected in the test functions themselves, not in a suite object.

Because of this, test suites are flexible and can have virtually any test attached to them depending on what is provided
in the initializer macro.

```rs
/// # TEST
/// Return a constant success!
fn always_succeed() -> ExtelResult {
    pass!()
}

/// # TEST
/// Return a constant failure... :(
fn always_fail() -> TestStatus {
    fail!("this test failed?")
}

fn main() {
    init_test_suite!(BasicTestSet, always_succeed, always_fail);
    BasicTestSet::run(TestConfig::default().output(OutputStyle::Stdout));
}
```

# Parameterized Testing
As of now Extel supports parameterized testing for single parameter functions. If you want to pass in multiple parameters,
wrap the input in a tuple or struct to test. You must add Extel with the `parameterized` feature enable.

```rs
/// # TEST
/// Check if the number is positive.
#[parameters(5, 10, -1)]
fn positive_num(n: usize) -> ExtelResult {
    extel_assert!(n > 0, "{} is not positive", n)
}

/// # TEST
/// Passing multiple args, check if lhs is greater than rhs.
#[parameters((4, 3), (4, 6))]
fn greater_than(nums: (usize, usize)) -> ExtelResult {
    let (lhs, rhs) = nums;
    extel_assert!(
        lhs > rhs,
        "left hand side is less than or equal to right hand side! ({} <= {})",
        lhs,
        rhs
    )
}

fn main() {
    init_test_suite!(ParameterizedTestSet, positive_num, greater_than);
    ParameterizedTestSet::run(TestConfig::default().output(OutputStyle::Stdout));
}
```
