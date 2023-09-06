use std::fmt::Display;

pub mod test_sets;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// Represents a test's success/fail status post-run.
pub enum TestStatus {
    Success,
    Fail(String),
}

/// A test instance that contains the test name and the test function that will be run.
pub struct Test {
    test_name: &'static str,
    test_fn: &'static dyn Fn() -> TestStatus,
}

impl Test {
    /// Run a test function, returning the name of the test and the result of it in a [TestResult].
    fn run_test(self) -> TestResult {
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

/// A test set that produces a list of test results.
pub trait RunnableTestSet {
    fn run() -> Vec<TestResult>;
}

#[macro_export]
macro_rules! init_tests {
    ($($test:expr),*) => {{
        let mut v: Vec<Test> = Vec::new();

        $(let test_name: &'static str = stringify!($test);
        let test_fn: &'static dyn Fn() -> TestStatus = &$test;
        v.push(Test { test_name, test_fn });)*

        v
    }};
}
