//! Test set for echo client & server.

#[cfg(test)]
mod tests {
    use std::process::Command;

    use crate::{init_tests, RunnableTestSet, Test, TestResult, TestStatus};

    pub struct EchoTestSet {
        pub tests: Vec<Test>,
    }

    impl EchoTestSet {
        fn initialize() -> Self {
            Self {
                tests: init_tests!(echo_hello_world, echo_hello_earth),
            }
        }
    }

    impl RunnableTestSet for EchoTestSet {
        fn run() -> Vec<TestResult> {
            EchoTestSet::initialize()
                .tests
                .into_iter()
                .map(Test::run_test)
                .collect()
        }
    }

    /// # TEST
    ///   - echo 'Hello, world!'
    fn echo_hello_world() -> TestStatus {
        let expected = String::from("Hello, world!");

        let output = Command::new("echo")
            .args(["-n", &expected])
            .output()
            .expect("could not execute echo");
        let string_output = String::from_utf8(output.stdout).expect("could not parse stdout");

        match &string_output == &expected {
            true => TestStatus::Success,
            false => TestStatus::Fail(format!(
                "mismatched output from echo: expected '{}', got '{}'",
                expected, string_output
            )),
        }
    }

    /// # TEST (SHOULD FAIL)
    ///   - echo 'Hello, earth!'
    fn echo_hello_earth() -> TestStatus {
        let wrong_msg = String::from("Hello, earth!");

        let output = Command::new("echo")
            .args(["-n", &wrong_msg])
            .output()
            .expect("could not execute echo");
        let string_output = String::from_utf8(output.stdout).expect("could not parse stdout");

        let expected = String::from("Hello, world!");
        match &string_output == &expected {
            true => TestStatus::Success,
            false => TestStatus::Fail(format!(
                "mismatched output from echo: expected '{}', got '{}'",
                expected, string_output
            )),
        }
    }

    #[test]
    fn run_all_echo_tests() {
        assert_eq!(
            EchoTestSet::run()
                .into_iter()
                .map(|res| res.test_result)
                .collect::<Vec<TestStatus>>(),
            vec![
                TestStatus::Success,
                TestStatus::Fail(String::from(
                    "mismatched output from echo: expected 'Hello, world!', got 'Hello, earth!'"
                ))
            ]
        );
    }
}
