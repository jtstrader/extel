//! A set of all `extel` macros.

#[cfg(not(doc))]
#[macro_export]
macro_rules! __extel_init_tests {
    ($($test:expr),*) => {{
        #[allow(unused_mut)]
        let mut v: Vec<$crate::Test> = Vec::new();

        $(let test_name: &'static str = stringify!($test);
        let test_fn: &'static dyn $crate::TestFunction = &($test as fn() -> $crate::ExtelResult);
        v.push($crate::Test { test_name, test_fn });)*

        v
    }};
}

/// A macro to create a passing ExtelResult. Under the hood this is represented as a
/// [`TestStatus`](crate::TestStatus) wrapped as a `Single` variant on
/// [`TestResultType`](crate::TestResultType).
///
/// # Example
/// ```rust
/// use extel::{pass, ExtelResult, TestResultType, TestStatus};
/// fn always_pass() -> ExtelResult {
///     pass!()
/// }
///
/// assert_eq!(
///     always_pass().get_test_result(),
///     TestResultType::Single(TestStatus::Success)
/// )
/// ```
#[macro_export]
macro_rules! pass {
    () => {
        Box::new($crate::TestStatus::Success)
    };
}

/// A macro to create a failing ExtelResult. Under the hood this is represented as a
/// [`TestStatus`](crate::TestStatus) wrapped as a `Single` variant on
/// [`TestResultType`](crate::TestResultType).
///
/// # Example
/// ```rust
/// use extel::{fail, ExtelResult, TestResultType, TestStatus};
/// fn always_fail() -> ExtelResult {
///     let error_msg = "this is an error message!";
///     fail!("This test fails with this error {}", error_msg)
/// }
///
/// assert_eq!(
///     always_fail().get_test_result(),
///     TestResultType::Single(
///         TestStatus::Fail(format!("This test fails with this error this is an error message!"))
///     )
/// )
/// ```
#[macro_export]
macro_rules! fail {
    ($fmt:expr, $($arg:expr),*) => {
        Box::new($crate::TestStatus::Fail(format!($fmt, $($arg),*)))
    };

    ($fmt:expr) => { Box::new($crate::TestStatus::Fail(format!($fmt))) }
}

/// Assert if a given condition is true/false. If the condition is true, call the [`pass`] macro,
/// else call the [`fail`] macro.
///
/// The fail macro can contain the following messages depending on which arm of the macro is used:
///   1. A default message such as "[condition] assertion failed."
///   2. A custom error message.
///   3. A custom format message that can take variable arguments.
///
/// This assertion is *not* like Rust's [`assert`] macro, and should not panic unless the given
/// condition or format arguments can panic. This macro returns an
/// [`ExtelResult`](crate::ExtelResult)
///
/// # Example
/// ```rust
/// use extel::extel_assert;
///
/// let (x, y, z) = (1, 1, 2);
///
/// // Resulting passes and error messages
/// extel_assert!(x == y);                         // passes -- no error message
/// extel_assert!(x == z);                         // "[x == z] assertion failed"
/// extel_assert!(x == z, "z was not 1!");         // "z was not 1!"
/// extel_assert!(y == z, "y = {}, z = {}", y, z); // "y = 1, z = 2"
/// ```
#[macro_export]
macro_rules! extel_assert {
    ($cond:expr) => {
        match $cond {
            true => $crate::pass!(),
            false => $crate::fail!("[{}] assertion failed", stringify!($cond)),
        }
    };

    ($cond:expr, $err:expr) => {
        match $cond {
            true => $crate::pass!(),
            false => $crate::fail!("{}", $err),
        }
    };

    ($cond:expr, $err_fmt:expr, $($arg:expr),+) => {
        extel_assert!($cond, format!($err_fmt, $($arg),+))
    }
}

/// Constructs a [`Command`](std::process::Command) as if receiving the command directly from the
/// CLI. Arguments wrapped in single or double quotes are treated as single arguments, allowing
/// multiple tokens to be passed as a single argument to a command.
///
/// # Example
/// ```rust
/// use extel::cmd;
///
/// const EXPECTED: &str = "hello world";
///
/// let cmd_output = cmd!("echo -n \"hello world\"").output().unwrap();
/// let cmd_output_fmt = cmd!("echo -n \"{}\"", EXPECTED).output().unwrap();
///
/// assert_eq!(
///     String::from_utf8_lossy(&cmd_output.stdout),
///     String::from_utf8_lossy(&cmd_output_fmt.stdout)
/// )
/// ```
#[macro_export]
macro_rules! cmd {
    ($cmd_str:expr) => {{
        // First, extract tokens by spliting them by spaces, but keep
        // together tokens that are wrapped in single/double quotes.
        let mut cmd_str_tokens = $cmd_str.trim().split(' ');
        let command = cmd_str_tokens.next().expect("no command was provided");
        let mut args = cmd_str_tokens.map(String::from);
        let mut final_args: Vec<String> = Vec::new();

        while let Some(token) = args.next() {
            // Get a token, check if quotes are present. If so, begin iterating
            // until a closing quote is found. If a closing quote is not found,
            // panic.
            let tok_chars = token.chars().collect::<Vec<_>>();
            let first_char = tok_chars[0];
            if ['"', '\''].contains(&first_char) {
                // Verify that the last token for this string is not also a quote.
                if *tok_chars.last().unwrap() == first_char {
                    final_args.push(tok_chars[1..tok_chars.len()-1].into_iter().collect());
                    break;
                }

                // Iterate until the next is found.
                let mut quoted_arg = vec![token.chars().skip(1).collect::<String>()];

                loop {
                    let Some(token) = args.next() else {
                        break;
                    };

                    // Check if the last char is a matching quote
                    let tok_chars = token.chars().collect::<Vec<_>>();
                    if *tok_chars.last().unwrap() == first_char {
                        quoted_arg.push(
                            // Assumes UTF-8
                            tok_chars[0..tok_chars.len()-1].into_iter().collect()
                        );
                    } else {
                        quoted_arg.push(token);
                    }
                }
                final_args.extend(quoted_arg);
            } else {
                final_args.push(token);
            }
        }

        let mut command = ::std::process::Command::new(command);
        if !final_args.is_empty() {
            command.args(final_args);
        }
        command
    }};

    ($cmd_str:expr, $($arg:expr),*) => {{
        let fmt = format!($cmd_str, $($arg),*);
        cmd!(fmt)
    }};
}

/// The test suite initializer that constructs test suits based on the provided name (first
/// parameter) and the provided functions (the comma-delimited list afterwards). Every function
/// that is provided is expected *only* to return type [`ExtelResult`](crate::ExtelResult), and
/// should have *no* parameters.
///
/// These tests are stateless in nature, relying on their environment and hard-coded CLI args to
/// handle configuration and valid setup.
///
/// # Example
/// ```rust
/// use std::process::Command;
/// use extel::prelude::*; 
///
/// /// Run end-to-end test of application.
/// fn echo_no_arg_e2e() -> ExtelResult {
///     match Command::new("echo").status() {
///         Ok(exit_code) => {
///             let code: i32 = exit_code.code().unwrap_or(-1);
///             extel_assert!(code == 0, "failed with exit code: {}", code)
///         },
///         Err(msg) => {
///             fail!("failed to execute with error: {}", msg)
///         }
///     }
/// }
///
/// // Outputs:
/// //  Test #1 (echo_no_arg_e2e) ... ok
/// init_test_suite!(EchoTestSuite, echo_no_arg_e2e);
/// EchoTestSuite::run(TestConfig::default());
/// ```
#[macro_export]
macro_rules! init_test_suite {
    ($test_suite:ident) => {
        init_test_suite!($test_suite,)
    };

    ($test_suite:ident, $($test_name:expr),*) => {
        #[allow(non_camel_case_types)]
        pub struct $test_suite {
            tests: Vec<$crate::Test>,
        }

        impl $crate::RunnableTestSet for $test_suite {
            fn run(cfg: $crate::TestConfig) -> Vec<$crate::TestResult> {
                let test_set = $test_suite { tests: $crate::__extel_init_tests!($($test_name),*) };
                let mut writer: Option<Box<dyn ::std::io::Write>> = match cfg.output {
                    $crate::OutputDest::Stdout => Some(Box::new(::std::io::stdout())),
                    $crate::OutputDest::File(file_name) => {
                        let file_handle = ::std::fs::File::create(file_name).expect("could not open output file");
                        Some(Box::new(file_handle))
                    },
                    $crate::OutputDest::Buffer(buffer) => Some(Box::new(buffer)),
                    $crate::OutputDest::None => None
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
                           $crate::output_test_result(w, &test_result, test_id + 1, cfg.colored);
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
    use crate::{ExtelResult, OutputDest, RunnableTestSet, TestConfig};

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
        BasicTestSet::run(
            TestConfig::default()
                .output(OutputDest::Buffer(output_buffer))
                .colored(false),
        );

        let output = String::from_utf8_lossy(output_buffer);

        assert_eq!(
            output,
            *"[extel::macros::tests::init_test_suite_basic::BasicTestSet]\n\t\
            Test #1 (always_succeed) ... ok\n\t\
            Test #2 (always_fail) ... FAILED\n\t  [x] this test failed?\n"
        );
    }

    #[test]
    fn test_cmd() {
        fn __test_cmd() -> ExtelResult {
            let output = cmd!("echo -n \"hello world\"")
                .output()
                .expect("could not execute command");
            let string_output =
                String::from_utf8(output.stdout).expect("output contained non-UTF-8 chars");

            // LEAVE AS pass!()/fail!() SYNTAX!
            // There is a test in this module that covers using extel_assert!. Keep these tests to
            // verify that pass!()/fail!() still functions as expected.
            if string_output == *"hello world" {
                pass!()
            } else {
                fail!(
                    "invalid result, expected 'hello world', got '{}'",
                    string_output
                )
            }
        }

        init_test_suite!(__test_cmd_suite, __test_cmd);
        let mut output_buffer: Vec<u8> = Vec::new();

        __test_cmd_suite::run(
            TestConfig::default()
                .output(OutputDest::Buffer(&mut output_buffer))
                .colored(false),
        );

        let output_result = String::from_utf8_lossy(&output_buffer);
        assert_eq!(
            output_result,
            "[extel::macros::tests::test_cmd::__test_cmd_suite]\n\tTest #1 (__test_cmd) ... ok\n"
        );
    }

    #[test]
    fn test_cmd_fmt_arg() {
        const EXPECTED: &str = "viva las vegas";
        fn __test_cmd() -> ExtelResult {
            let output = cmd!("echo -n \"{}\"", EXPECTED)
                .output()
                .expect("could not execute command");
            let string_output =
                String::from_utf8(output.stdout).expect("output contained non-UTF-8 chars");

            // LEAVE AS pass!()/fail!() SYNTAX!
            // There is a test in this module that covers using extel_assert!. Keep these tests to
            // verify that pass!()/fail!() still functions as expected.
            if string_output == *EXPECTED {
                pass!()
            } else {
                fail!(
                    "invalid result, expected '{}', got '{}'",
                    EXPECTED,
                    string_output
                )
            }
        }

        init_test_suite!(__test_cmd_suite, __test_cmd);
        let mut output_buffer: Vec<u8> = Vec::new();

        __test_cmd_suite::run(
            TestConfig::default()
                .output(OutputDest::Buffer(&mut output_buffer))
                .colored(false),
        );

        let output_result = String::from_utf8_lossy(&output_buffer);
        assert_eq!(
            output_result,
            "[extel::macros::tests::test_cmd_fmt_arg::__test_cmd_suite]\n\tTest #1 (__test_cmd) ... ok\n"
        );
    }

    #[test]
    fn test_extel_assert() {
        const EXPECTED: &str = "viva las vegas";
        fn __test_cmd() -> ExtelResult {
            let output = cmd!("echo -n \"{}\"", EXPECTED)
                .output()
                .expect("could not execute command");
            let string_output =
                String::from_utf8(output.stdout).expect("output contained non-UTF-8 chars");

            extel_assert!(
                string_output == *EXPECTED,
                "invalid result, expected '{}', got '{}'",
                EXPECTED,
                string_output
            )
        }

        init_test_suite!(__test_cmd_suite, __test_cmd);
        let mut output_buffer: Vec<u8> = Vec::new();

        __test_cmd_suite::run(
            TestConfig::default()
                .output(OutputDest::Buffer(&mut output_buffer))
                .colored(false),
        );

        let output_result = String::from_utf8_lossy(&output_buffer);
        assert_eq!(
            output_result,
            "[extel::macros::tests::test_extel_assert::__test_cmd_suite]\n\tTest #1 (__test_cmd) ... ok\n"
        );
    }
}
