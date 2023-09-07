use std::{
    fmt::Display,
    io::{BufWriter, Write},
};

pub mod macros;
pub mod test_sets;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// Represents a test's success/fail status post-run.
pub enum TestStatus {
    Success,
    Fail(String),
}

/// A test instance that contains the test name and the test function that will be run.
pub struct Test {
    pub test_name: &'static str,
    pub test_fn: &'static dyn Fn() -> TestStatus,
}

impl Test {
    /// Run a test function, returning the name of the test and the result of it in a [`TestResult`].
    pub fn run_test(self) -> TestResult {
        TestResult {
            test_name: self.test_name,
            test_result: (self.test_fn)(),
        }
    }
}

/// A test result item that contains the name of the test and a result value. The value can either
/// be a success or a failure. If a failure, there will be an underlying message as well to explain
/// the context of the failure.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TestResult {
    pub test_name: &'static str,
    pub test_result: TestStatus,
}

impl Display for TestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TestStatus::Success => String::from("OK"),
                TestStatus::Fail(msg) => format!("FAIL\n\n\t\t{}\n", msg),
            }
        )
    }
}

/// The output method for logging test results.
#[derive(Debug)]
pub enum OutputStyle<'a> {
    Stdout,
    File(&'static str),
    Buffer(&'a mut Vec<u8>),
    None,
}

/// A test configuration type that determines what features will be enabled on the tests.
#[derive(Debug)]
pub struct TestConfig<'a> {
    pub output: OutputStyle<'a>,
}

impl<'a> TestConfig<'a> {
    pub fn output(mut self, output_style: OutputStyle<'a>) -> Self {
        self.output = output_style;
        self
    }
}

impl<'a> Default for TestConfig<'a> {
    fn default() -> Self {
        Self {
            output: OutputStyle::Stdout,
        }
    }
}

/// A test set that produces a list of test results.
pub trait RunnableTestSet {
    fn run(cfg: TestConfig) -> Vec<TestResult>;
}

pub fn output_test_result<T>(stream: T, result: &TestResult, test_num: usize)
where
    T: Write,
{
    let fmt_output = match &result.test_result {
        TestStatus::Success => format!("\tTest #{} ({}): OK\n", test_num, result.test_name),
        TestStatus::Fail(err_msg) => format!(
            "\tTest #{} ({}): FAIL\n\n\t\t{}\n\n",
            test_num, result.test_name, err_msg
        ),
    };

    let mut writer: BufWriter<T> = BufWriter::new(stream);
    writer
        .write_all(fmt_output.as_bytes())
        .expect("stream could not be written to");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_test_output() {
        let ok_test = TestResult {
            test_name: "this_test_passes",
            test_result: TestStatus::Success,
        };

        let fail_test = TestResult {
            test_name: "this_test_fails",
            test_result: TestStatus::Fail(format!("test failed after {}", ok_test.test_name)),
        };

        let mut ok_result_buffer: Vec<u8> = Vec::new();
        let mut fail_result_buffer: Vec<u8> = Vec::new();

        output_test_result(&mut ok_result_buffer, &ok_test, 1);
        output_test_result(&mut fail_result_buffer, &fail_test, 2);

        assert_eq!(
            String::from_utf8_lossy(&ok_result_buffer),
            "\tTest #1 (this_test_passes): OK\n"
        );

        assert_eq!(
            String::from_utf8_lossy(&fail_result_buffer),
            "\tTest #2 (this_test_fails): FAIL\n\n\t\ttest failed after this_test_passes\n\n"
        );
    }
}
