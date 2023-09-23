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
//! use extel::{pass, fail, cmd, init_test_suite, ExtelResult, RunnableTestSet, TestConfig};
//! use extel_parameterized::parameters;
//!
//! fn single_test() -> ExtelResult {
//!     let mut my_cmd = cmd!("echo -n \"hello world\"");
//!     let output = my_cmd.output().unwrap();
//!     let string_output = String::from_utf8_lossy(&output.stdout);
//!
//!     if string_output == *"hello world" {
//!         pass!()
//!     } else {
//!         fail!("expected 'hello world', got '{}'", string_output)
//!     }
//! }
//!
//! #[parameters(1, 2, -2, 4)]
//! fn param_test(x: i32) -> ExtelResult {
//!     if x >= 0 {
//!         pass!()
//!     } else {
//!         fail!("{} < 0", x)
//!     }
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
/// use extel::{fail, pass, ExtelResult, TestResultType, TestStatus};
/// use extel_parameterized::parameters;
///
/// #[parameters(2, 4)]
/// fn less_than_3(x: i32) -> ExtelResult {
///     match x < 3 {
///         true => pass!(),
///         false => fail!("{} >= 3", x),
///     }
/// }
///
/// assert_eq!(
///     less_than_3().get_test_result(),
///     TestResultType::Parameterized(vec![
///         TestStatus::Success,
///         TestStatus::Fail(String::from("4 >= 3"))
///     ])
/// );
/// ```
/// > *This is only available with the `parameterized` feature enabled.*
#[cfg(feature = "parameterized")]
pub use extel_parameterized::parameters;

pub mod prelude {
    pub use crate::{cmd, fail, init_test_suite, pass, ExtelResult, RunnableTestSet, TestConfig};

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
    /// use extel::{fail, pass, ExtelResult, TestResultType, TestStatus};
    /// use extel_parameterized::parameters;
    ///
    /// #[parameters(2, 4)]
    /// fn less_than_3(x: i32) -> ExtelResult {
    ///     match x < 3 {
    ///         true => pass!(),
    ///         false => fail!("{} >= 3", x),
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     less_than_3().get_test_result(),
    ///     TestResultType::Parameterized(vec![
    ///         TestStatus::Success,
    ///         TestStatus::Fail(String::from("4 >= 3"))
    ///     ])
    /// );
    /// ```
    /// > *This is only available with the `parameterized` feature enabled.*
    #[cfg(feature = "parameterized")]
    pub use extel_parameterized::parameters;
}

use std::io::{BufWriter, Write};

#[doc(hidden)]
pub mod macros;

/// For verifying/testing Extel.
mod test_sets;

/// The expected return type of extel test functions. This type is a generic type to wrap around
/// both standard (single) and parameterized tests. The easiest way to create these results is to
/// use the [`pass`] and [`fail`] macros.
///
/// To get the underlying test result variant, use the
/// [`get_test_result`](GenericTestResult::get_test_result) function.
///
/// # Example
/// ```rust
/// use extel::{pass, fail, ExtelResult, TestStatus, TestResultType};
///
/// fn always_succeed() -> ExtelResult {
///     pass!()
/// }
///
/// let res = always_succeed().get_test_result();
/// assert_eq!(res, TestResultType::Single(TestStatus::Success));
/// ```
pub type ExtelResult = Box<dyn crate::GenericTestResult>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// Represents a test's success/fail status post-run.
pub enum TestStatus {
    Success,
    Fail(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// A test result variant that helps distinguish between standard, or single, tests and
/// parameterized tests. Both the `Single` and `Parameterized` variants contain one or more
/// [`TestStatus`] structs.
pub enum TestResultType {
    Single(TestStatus),
    Parameterized(Vec<TestStatus>),
}

/// Represents a generic test result. The test result can be extracted into a [`TestResultType`] to
/// determine if the result came from a parameterized or single test.
pub trait GenericTestResult {
    fn get_test_result(&self) -> TestResultType;
}

impl TryInto<TestStatus> for ExtelResult {
    type Error = &'static str;

    /// Try to convert an ExtelResult into a single test result. Will return an error if the result
    /// passed in is a parameterized result.
    fn try_into(self) -> Result<TestStatus, Self::Error> {
        // Note: internally this may seem purposeless, but this is for ease of use! No test written
        // by the user can feasibly return a TestResultType::Single unless writing a parameterized
        // test. In the end this just helps organize and cleanup some internal code that may need
        // to extract an ExtelResult into a specific variant.
        match self.get_test_result() {
            TestResultType::Single(result) => Ok(result),
            _ => Err("cannot call `into` on parameterized result"),
        }
    }
}

impl GenericTestResult for TestStatus {
    fn get_test_result(&self) -> TestResultType {
        TestResultType::Single(self.clone())
    }
}

impl GenericTestResult for Vec<TestStatus> {
    fn get_test_result(&self) -> TestResultType {
        TestResultType::Parameterized(self.clone())
    }
}

/// A trait for basic, parameterless function types that return [`ExtelResult`].
pub trait TestFunction {
    fn run_test_fn(&self) -> TestResultType;
}

impl TestFunction for fn() -> ExtelResult {
    fn run_test_fn(&self) -> TestResultType {
        self().get_test_result()
    }
}

/// A test instance that contains the test name and the test function that will be run.
pub struct Test {
    pub test_name: &'static str,
    pub test_fn: &'static dyn TestFunction,
}

impl Test {
    /// Run a test function, returning the name of the test and the result of it in a [`TestResult`].
    pub fn run_test(self) -> TestResult {
        TestResult {
            test_name: self.test_name,
            test_result: (self.test_fn).run_test_fn(),
        }
    }
}

/// A test result item that contains the name of the test and a result value. The value can either
/// be a success or a failure. If a failure, there will be an underlying message as well to explain
/// the context of the failure.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TestResult {
    pub test_name: &'static str,
    pub test_result: TestResultType,
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
/// output, consider [RunnableTestSet::run].
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
        TestResultType::Single(status) => match status {
            TestStatus::Success => format!(
                "\tTest #{} ({}) ... {ok_color}ok{color_terminator}\n",
                test_num, result.test_name
            ),
            TestStatus::Fail(err_msg) => format!(
                "\tTest #{} ({}) ... {fail_color}FAILED{color_terminator}\n\t  [x] {}\n",
                test_num, result.test_name, err_msg
            ),
        },
        TestResultType::Parameterized(statuses) => statuses
            .iter()
            .enumerate()
            .map(|(idx, status)| match status {
                TestStatus::Success => {
                    format!(
                        "\tTest #{}.{} ({}) ... {ok_color}ok{color_terminator}\n",
                        test_num, idx, result.test_name
                    )
                }
                TestStatus::Fail(err_msg) => format!(
                    "\tTest #{}.{} ({}) ... {fail_color}FAILED{color_terminator}\n\t  [x] {}\n",
                    test_num,
                    idx + 1,
                    result.test_name,
                    err_msg
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
    use TestResultType as TRT;

    #[test]
    fn write_test_output_no_color() {
        let ok_test = TestResult {
            test_name: "this_test_passes",
            test_result: TRT::Single(TestStatus::Success),
        };

        let fail_test = TestResult {
            test_name: "this_test_fails",
            test_result: TRT::Single(TestStatus::Fail(format!(
                "test failed after {}",
                ok_test.test_name
            ))),
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
            test_result: TRT::Single(TestStatus::Success),
        };

        let fail_test = TestResult {
            test_name: "this_test_fails",
            test_result: TRT::Single(TestStatus::Fail(format!(
                "test failed after {}",
                ok_test.test_name
            ))),
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
