//! A set of all `extel` macros.

#[macro_export]
macro_rules! init_tests {
    ($($test:expr),*) => {{
        #[allow(unused_mut)]
        let mut v: Vec<$crate::Test> = Vec::new();

        $(let test_name: &'static str = stringify!($test);
        let test_fn: &'static dyn $crate::TestFunction = &($test as fn() -> $crate::ExtelResult);
        v.push($crate::Test { test_name, test_fn });)*

        v
    }};
}

#[macro_export]
macro_rules! pass {
    () => {
        Box::new($crate::TestStatus::Success)
    };
}

#[macro_export]
macro_rules! fail {
    ($fmt:expr, $($arg:expr),*) => {
        Box::new($crate::TestStatus::Fail(format!($fmt, $($arg),*)))
    };

    ($fmt:expr) => { Box::new($crate::TestStatus::Fail(format!($fmt)))}
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
/// use extel::{fail, init_test_suite, pass, ExtelResult, TestConfig, RunnableTestSet};
///
/// /// Run end-to-end test of application.
/// fn echo_no_arg_e2e() -> ExtelResult {
///     match Command::new("echo").status() {
///         Ok(exit_code) => {
///             let code: i32 = exit_code.code().unwrap_or(-1);
///             if code == 0 {
///                 pass!()
///             } else {
///                 fail!("failed with exit code {}", code)
///             }
///         },
///         Err(msg) => {
///             fail!("failed to execute with error: {}", msg)
///         }
///     }
/// }
///
/// // Outputs:
/// //  Test #1 (echo_no_arg_e2e): OK
/// init_test_suite!(EchoTestSuite, echo_no_arg_e2e);
/// EchoTestSuite::run(TestConfig::default());
/// ```
#[macro_export]
macro_rules! init_test_suite {
    ($test_suite:ident) => {
        init_test_suite!($test_suite,)
    };

    ($test_suite:ident, $($test_name:expr),*) => {
        pub struct $test_suite {
            tests: Vec<$crate::Test>,
        }

        impl $crate::RunnableTestSet for $test_suite {
            fn run(cfg: $crate::TestConfig) -> Vec<$crate::TestResult> {
                let test_set = $test_suite { tests: $crate::init_tests!($($test_name),*) };
                let mut writer: Option<Box<dyn ::std::io::Write>> = match cfg.output {
                    $crate::OutputStyle::Stdout => Some(Box::new(::std::io::stdout())),
                    $crate::OutputStyle::File(file_name) => {
                        let file_handle = ::std::fs::File::create(file_name).expect("could not open output file");
                        Some(Box::new(file_handle))
                    },
                    $crate::OutputStyle::Buffer(buffer) => Some(Box::new(buffer)),
                    $crate::OutputStyle::None => None
                };

                if let Some(w) = writer.as_mut() {
                    write!(w, "[{}]\n", std::any::type_name::<$test_suite>()).expect("buffer could not be written to");
                }

                // Begin running tests and logging to the desired writer
                test_set
                    .tests
                    .into_iter()
                    .enumerate()
                    .map(|(test_id, test)| {
                        let test_result = test.run_test();

                        if let Some(w) = writer.as_mut() {
                           $crate::output_test_result(w, &test_result, test_id + 1);
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
    use crate::{ExtelResult, OutputStyle, RunnableTestSet, TestConfig, TestStatus};

    /// # TEST
    ///   - Return a constant success!
    fn always_succeed() -> ExtelResult {
        pass!()
    }

    /// # TEST
    ///   - Return a constant failure... :(
    fn always_fail() -> ExtelResult {
        fail!("this test failed?")
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
            *"[extel::macros::tests::init_test_suite_basic::BasicTestSet]\n\tTest #1 (always_succeed): OK\n\tTest #2 (always_fail): FAIL\n\n\t\tthis test failed?\n\n"
        );
    }
}
