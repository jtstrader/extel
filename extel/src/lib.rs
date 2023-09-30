//! ## Extel - Extended Testing Library
//! Extel is a testing library that intends to help create scalable test suites with stateless
//! tests. Extel's primary purpose is to make writing tests fast, easy, and performant. A common
//! use case for Extel is when writing integration tests for external binaries/executables. Extel
//! comes with some macros for creating and parsing command results to streamline the process of
//! creating and running CLI commands.
//!
//! Think of Extel like a scripting language for tests. There's no need for importing a set of
//! wacky modules, or writing weird redirections in a shell language to capture stdout. Instead,
//! you can use Extel to save the output or status results of the commands you want to run, and
//! then chose where your test output goes, too!
//!
//! ## Usage
//! Extel is intended to function on zero argument or single argument functions (the former being
//! called *single* tests and the latter *parameterized* tests). After creating your test function,
//! you can register it using the [`init_test_suite`] macro. This will scaffold a struct containing
//! pointers to the functions passed in. Calling the [`run`](RunnableTestSet::run) function on the
//! generated struct will immediately go through all tests and collect the results into a vector
//! for the user to parse through if they so wish.
//!
//! Note that all Extel test functions expect an [`ExtelResult`] in its return. Using another type
//! will cause the macros generating the test suite to fail, or will return some weird errors while
//! using the `parameters` proc macro.
//!
//! ```rust
//! use extel::prelude::*;
//! use extel_parameterized::parameters;
//!
//! fn single_test() -> ExtelResult {
//!     let mut my_cmd = cmd!("echo -n \"hello world\"");
//!     let output = my_cmd.output()?;
//!     let string_output = String::from_utf8(output.stdout)?;
//!
//!     extel_assert!(
//!         string_output == *"hello world",
//!         "expected 'hello world', got '{}'",
//!         string_output
//!     )
//! }
//!
//! #[parameters(1, 2, -2, 4)]
//! fn param_test(x: i32) -> ExtelResult {
//!     extel_assert!(x >= 0, "{} < 0", x)
//! }
//!
//! fn main() {
//!     init_test_suite!(ExtelDemo, single_test, param_test);
//!     ExtelDemo::run(TestConfig::default());
//! }

/// Convert a *single argument function* into a parameterized function. The expected function
/// signature is a single argument function (can be any type) that returns an
/// [`ExtelResult`](crate::ExtelResult).
///
/// While technically possible, this macro is not intended to be used to run tests
/// manually. This macro is specifically for the purpose of helping the [test
/// initializer](crate::init_test_suite) prepare parameterized tests.
///
/// # Example
/// ```rust
/// use extel::prelude::*;
/// use extel_parameterized::parameters;
///
/// #[parameters(2, 4)]
/// fn less_than_3(x: i32) -> ExtelResult {
///     extel_assert!(x < 3, "{} >= 3", x)
/// }
///
/// assert!(matches!(
///     &less_than_3()[..],
///     [
///         Ok(_),
///         Err(Error::TestFailed(_))
///     ]
/// ));
/// ```
/// > *This is only available with the `parameterized` feature enabled.*
#[cfg(feature = "parameterized")]
pub use extel_parameterized::parameters;

pub mod prelude {
    pub use crate::{
        cmd, errors::Error, extel_assert, fail, init_test_suite, pass, ExtelResult,
        RunnableTestSet, TestConfig,
    };

    /// Convert a *single argument function* into a parameterized function. The expected function
    /// signature is a single argument function (can be any type) that returns an
    /// [`ExtelResult`](crate::ExtelResult).
    ///
    /// While technically possible, this macro is not intended to be used to run tests
    /// manually. This macro is specifically for the purpose of helping the [test
    /// initializer](crate::init_test_suite) prepare parameterized tests.
    ///
    /// # Example
    /// ```rust
    /// use extel::prelude::*;
    /// use extel_parameterized::parameters;
    ///
    /// #[parameters(2, 4)]
    /// fn less_than_3(x: i32) -> ExtelResult {
    ///     extel_assert!(x < 3, "{} >= 3", x)
    /// }
    ///
    /// assert!(matches!(
    ///     &less_than_3()[..],
    ///     [
    ///         Ok(_),
    ///         Err(Error::TestFailed(_))
    ///     ]
    /// ));
    /// ```
    /// > *This is only available with the `parameterized` feature enabled.*
    #[cfg(feature = "parameterized")]
    pub use extel_parameterized::parameters;
}

use errors::Error;
use std::io::{BufWriter, Write};

pub mod errors;

#[doc(hidden)]
pub mod macros;

/// The expected return type of extel test functions. This type is represented as a result type to
/// allow error propogation.
///
/// # Example
/// ```rust
/// use extel::{pass, fail, ExtelResult};
///
/// fn always_succeed() -> ExtelResult {
///     pass!()
/// }
///
/// fn always_fail() -> ExtelResult {
///     fail!("failed")
/// }
///
/// fn early_fail_from_err() -> ExtelResult {
///     let invalid_utf8 = *b"\xE0\x80\x80";
///     let _ = String::from_utf8(invalid_utf8.into())?;
///     pass!()
/// }
///
/// assert_eq!(
///     vec![true, false, false],
///     vec![
///         always_succeed().is_ok(),
///         always_fail().is_ok(),
///         early_fail_from_err().is_ok()
///     ]
/// );
/// ```
pub type ExtelResult = Result<(), Error>;

#[derive(Debug)]
/// A test result variant that helps distinguish between standard, or single, tests and
/// parameterized tests. Both the `Single` and `Parameterized` variants contain one or more
/// [`ExtelResult`].
pub enum TestStatus {
    Single(ExtelResult),
    Parameterized(Vec<ExtelResult>),
}

/// Represents a generic test result. The test result can be extracted into a [`TestStatus`] to
/// determine if the result came from a parameterized or single test.
pub trait GenericTestResult {
    fn get_test_result(self: Box<Self>) -> TestStatus;
}

impl GenericTestResult for ExtelResult {
    fn get_test_result(self: Box<Self>) -> TestStatus {
        TestStatus::Single(*self)
    }
}

impl GenericTestResult for Vec<ExtelResult> {
    fn get_test_result(self: Box<Self>) -> TestStatus {
        TestStatus::Parameterized(*self)
    }
}

/// A test instance that contains the test name and the test function that will be run.
pub struct Test {
    pub test_name: &'static str,
    pub test_fn: fn() -> Box<dyn GenericTestResult>,
}

impl Test {
    /// Run a test function, returning the name of the test and the result of it in a [`GenericTestResult`].
    pub fn run_test(self) -> TestResult {
        TestResult {
            test_name: self.test_name,
            test_result: (self.test_fn)().get_test_result(),
        }
    }
}

/// A test result item that contains the name of the test and a result value. The value can either
/// be a success or a failure. If a failure, there will be an underlying message as well to explain
/// the context of the failure.
#[derive(Debug)]
pub struct TestResult {
    pub test_name: &'static str,
    pub test_result: TestStatus,
}

/// The output method for logging test results.
#[derive(Debug)]
pub enum OutputDest<'a> {
    Stdout,
    File(&'static str),
    Buffer(&'a mut Vec<u8>),
    None,
}

/// A test configuration type that determines what features will be enabled on the tests.
#[derive(Debug)]
pub struct TestConfig<'a> {
    pub output: OutputDest<'a>,
    pub colored: bool,
}

impl<'a> TestConfig<'a> {
    /// Change the output destination.
    pub fn output(mut self, output_style: OutputDest<'a>) -> Self {
        self.output = output_style;
        self
    }

    /// Change whether or not the logging should output with ANSI color codes.
    pub fn colored(mut self, yes: bool) -> Self {
        self.colored = yes;
        self
    }
}

impl<'a> Default for TestConfig<'a> {
    fn default() -> Self {
        Self {
            output: OutputDest::Stdout,
            colored: true,
        }
    }
}

/// A test set that produces a list of test results.
pub trait RunnableTestSet {
    /// Run a test set with the provided configuration to create a list of test results. The test
    /// suite can contain both single, or standard, tests and parameterized tests. The results of
    /// the parameterized tests will be flattened into the resulting vec.
    fn run(cfg: TestConfig) -> Vec<TestResult>;
}

/// Output the test results to the desired stream. This function is public only to give
/// availability to the [test initializer](crate::init_test_suite). If you wish to generate test
/// output, consider [`RunnableTestSet::run`].
pub fn output_test_result<T: Write>(
    stream: T,
    result: &TestResult,
    test_num: usize,
    colored: bool,
) {
    // Kinda bogus but it'll work :V
    let color_terminator = match colored {
        true => "\x1b[0m",
        false => "",
    };
    let ok_color = match colored {
        true => "\x1b[32m",
        false => "",
    };
    let fail_color = match colored {
        true => "\x1b[31m",
        false => "",
    };

    let fmt_output = match &result.test_result {
        TestStatus::Single(status) => match &*status {
            Ok(()) => format!(
                "\tTest #{} ({}) ... {ok_color}ok{color_terminator}\n",
                test_num, result.test_name
            ),
            Err(err_msg) => format!(
                "\tTest #{} ({}) ... {fail_color}FAILED{color_terminator}\n\t  [x] {}\n",
                test_num,
                result.test_name,
                err_msg.to_string()
            ),
        },
        TestStatus::Parameterized(statuses) => statuses
            .iter()
            .enumerate()
            .map(|(idx, status)| match status {
                Ok(()) => {
                    format!(
                        "\tTest #{}.{} ({}) ... {ok_color}ok{color_terminator}\n",
                        test_num, idx, result.test_name
                    )
                }
                Err(err_msg) => format!(
                    "\tTest #{}.{} ({}) ... {fail_color}FAILED{color_terminator}\n\t  [x] {}\n",
                    test_num,
                    idx + 1,
                    result.test_name,
                    err_msg.to_string()
                ),
            })
            .collect::<String>(),
    };

    let mut writer: BufWriter<T> = BufWriter::new(stream);
    writer
        .write_all(fmt_output.as_bytes())
        .expect("stream could not be written to");
}

#[cfg(test)]
mod tests {
    use super::*;
    use Error as XE;
    use TestStatus as TRT;

    #[test]
    fn write_test_output_no_color() {
        let ok_test = TestResult {
            test_name: "this_test_passes",
            test_result: TRT::Single(Ok(())),
        };

        let fail_test = TestResult {
            test_name: "this_test_fails",
            test_result: TRT::Single(Err(XE::TestFailed(format!(
                "test failed after {}",
                ok_test.test_name
            )))),
        };

        let mut ok_result_buffer: Vec<u8> = Vec::new();
        let mut fail_result_buffer: Vec<u8> = Vec::new();

        output_test_result(&mut ok_result_buffer, &ok_test, 1, false);
        output_test_result(&mut fail_result_buffer, &fail_test, 2, false);

        assert_eq!(
            String::from_utf8_lossy(&ok_result_buffer),
            "\tTest #1 (this_test_passes) ... ok\n"
        );

        assert_eq!(
            String::from_utf8_lossy(&fail_result_buffer),
            "\tTest #2 (this_test_fails) ... FAILED\n\t  [x] test failed after this_test_passes\n"
        );
    }

    #[test]
    fn write_test_output_with_color() {
        let ok_test = TestResult {
            test_name: "this_test_passes",
            test_result: TRT::Single(Ok(())),
        };

        let fail_test = TestResult {
            test_name: "this_test_fails",
            test_result: TRT::Single(Err(XE::TestFailed(format!(
                "test failed after {}",
                ok_test.test_name
            )))),
        };

        let mut ok_result_buffer: Vec<u8> = Vec::new();
        let mut fail_result_buffer: Vec<u8> = Vec::new();

        output_test_result(&mut ok_result_buffer, &ok_test, 1, true);
        output_test_result(&mut fail_result_buffer, &fail_test, 2, true);

        assert_eq!(
            String::from_utf8_lossy(&ok_result_buffer),
            "\tTest #1 (this_test_passes) ... \x1b[32mok\x1b[0m\n"
        );

        assert_eq!(
            String::from_utf8_lossy(&fail_result_buffer),
            "\tTest #2 (this_test_fails) ... \x1b[31mFAILED\x1b[0m\n\t  [x] test failed after this_test_passes\n"
        );
    }
}
