//! A set of all `extel` macros.

#[macro_export]
macro_rules! init_tests {
    ($($test:expr),*) => {{
        #[allow(unused_mut)]
        let mut v: Vec<Test> = Vec::new();

        $(let test_name: &'static str = stringify!($test);
        let test_fn: &'static dyn Fn() -> TestStatus = &$test;
        v.push(Test { test_name, test_fn });)*

        v
    }};
}

/// The test suite initializer that constructs test suits based on the provided name (first
/// parameter) and the provided functions (the comma-delimited list afterwards). Every function
/// that is provided is expected *only* to return type [`TestStatus`](crate::TestStatus), and
/// should have *no* parameters.
///
/// These tests are stateless in nature, relying on their environment and hard-coded CLI args to
/// handle configuration and valid setup.
///
/// # Example
/// ```rust
/// use std::process::Command;
/// use extel::{init_tests, init_test_suite, TestStatus};
///
/// /// Run end-to-end test of application.
/// fn ls_no_arg_e2e() -> TestStatus {
///     match Command::new("echo").status() {
///         Ok(exit_code) => {
///             let code: i32 = exit_code.code().unwrap_or(-1);
///             if code == 0 {
///                 TestStatus::Success
///             } else {
///                 TestStatus::Fail(format!("failed with exit code {}", code))
///             }
///         },
///         Err(msg) => {
///             TestStatus::Fail(format!("failed to execute with error: {}", msg))
///         }
///     }
/// }
///
/// // Outputs:
/// //  Test #1 (ls_no_arg_e2e): OK
/// init_test_suite!(LsTestSuite, ls_no_arg_e2e);
/// LsTestSuite::run(TestConfig::default());
/// ```
#[macro_export]
macro_rules! init_test_suite {
    ($test_suite:ident) => {
        init_test_suite!($test_suite,)
    };

    ($test_suite:ident, $($test_name:expr),*) => {
        use $crate::{RunnableTestSet, Test, TestConfig, TestResult, OutputStyle, output_test_result};

        pub struct $test_suite {
            tests: Vec<Test>,
        }

        impl RunnableTestSet for $test_suite {
            fn run(cfg: TestConfig) -> Vec<TestResult> {
                let test_set = $test_suite { tests: init_tests!($($test_name),*) };
                let mut writer: Option<Box<dyn ::std::io::Write>> = match cfg.output {
                    OutputStyle::Stdout => Some(Box::new(::std::io::stdout())),
                    OutputStyle::File(file_name) => {
                        let file_handle = ::std::fs::File::create(file_name).expect("could not open output file");
                        Some(Box::new(file_handle))
                    },
                    OutputStyle::Buffer(buffer) => Some(Box::new(buffer)),
                    OutputStyle::None => None
                };

                // Begin running tests and logging to the desired writer
                test_set
                    .tests
                    .into_iter()
                    .enumerate()
                    .map(|(test_id, test)| {
                        let test_result = test.run_test();

                        if let Some(w) = writer.as_mut() {
                            output_test_result(w, &test_result, test_id + 1);
                        }

                        test_result
                    })
                    .collect()
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::TestStatus;

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

    #[test]
    fn init_test_suite_basic() {
        init_test_suite!(BasicTestSet, always_succeed, always_fail);

        // Create output buffer
        let output_buffer: &mut Vec<u8> = &mut Vec::new();
        BasicTestSet::run(TestConfig::default().output(OutputStyle::Buffer(output_buffer)));

        let output = String::from_utf8_lossy(output_buffer);

        assert_eq!(
            output,
            *"Test #1 (always_succeed): OK\nTest #2 (always_fail): FAIL\n\n\tthis test failed?\n\n"
        );
    }
}
