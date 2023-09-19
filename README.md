# Extel - Extendable Testing Library
Extel is a testing library for Rust that supports both unit/integration testing for Rust projects and external APIs
through FFI/`std::process`.

# Using Extel
Extel is designed to work with *stateless functions*. Originally built to test executables built in C, Extel follows the
philosphy of both unit and integration tests not requiring any internal state. If any parameters or arguments are needed
specifically for testing external APIs, these should be reflected in the test functions themselves, not in a suite object.

Because of this, test suites are flexible and can have virtually any test attached to them depending on what is provided
in the initializer macro.

```rs
/// # TEST
///   - Return a constant success!
fn always_succeed() -> TestStatus {
    TestStatus::Success
}

/// # TEST
///   - Return a constant failure... :(
fn always_fail() -> TestStatus {
    TestStatus::Fail("this test failed?".to_string())
}

fn main() {
    init_test_suite!(BasicTestSet, always_succeed, always_fail);
    BasicTestSet::run(TestConfig::default().output(OutputStyle::Stdout));
}
```
