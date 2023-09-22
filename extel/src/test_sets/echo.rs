//! Test set for echo client & server.

#[cfg(test)]
mod tests {
    use std::process::Command;

    use crate::{
        fail, init_test_suite, pass, ExtelResult, OutputStyle, RunnableTestSet, TestConfig,
        TestResultType, TestStatus,
    };

    /// # TEST
    ///   - echo 'Hello, world!'
    fn echo_hello_world() -> ExtelResult {
        let expected = String::from("Hello, world!");

        let output = Command::new("echo")
            .args(["-n", &expected])
            .output()
            .expect("could not execute echo");
        let string_output = String::from_utf8(output.stdout).expect("could not parse stdout");

        match string_output == expected {
            true => pass!(),
            false => fail!(
                "mismatched output from echo: expected '{}', got '{}'",
                expected,
                string_output
            ),
        }
    }

    /// # TEST (SHOULD FAIL)
    ///   - echo 'Hello, earth!'
    fn echo_hello_earth() -> ExtelResult {
        let wrong_msg = String::from("Hello, earth!");

        let output = Command::new("echo")
            .args(["-n", &wrong_msg])
            .output()
            .expect("could not execute echo");
        let string_output = String::from_utf8(output.stdout).expect("could not parse stdout");

        let expected = String::from("Hello, world!");
        match string_output == expected {
            true => pass!(),
            false => fail!(
                "mismatched output from echo: expected '{}', got '{}'",
                expected,
                string_output
            ),
        }
    }

    /// # TEST
    ///   - echo something, look for a success code
    fn echo_anything() -> ExtelResult {
        match Command::new("echo").arg("-n").status() {
            Ok(exit_code) => exit_code.code().map_or_else(
                || fail!("no exit code found"),
                |code| {
                    if code == 0 {
                        pass!()
                    } else {
                        fail!("failed with exit code: {}", code)
                    }
                },
            ),
            Err(e) => fail!("could not execute with error: {}", e),
        }
    }

    #[test]
    fn echo_test_set() {
        let output_buffer: &mut Vec<u8> = &mut Vec::new();

        init_test_suite!(EchoTestSet, echo_hello_world, echo_hello_earth);
        EchoTestSet::run(TestConfig::default().output(OutputStyle::Buffer(output_buffer)));

        let output = String::from_utf8_lossy(output_buffer);
        assert_eq!(
            output,
            "[extel::test_sets::echo::tests::echo_test_set::EchoTestSet]\n\tTest #1 (echo_hello_world): OK\n\
            \tTest #2 (echo_hello_earth): FAIL\n\n\t\tmismatched output from echo: expected 'Hello, world!', got 'Hello, earth!'\n\n"
        );
    }

    #[test]
    fn echo_anything_many() {
        init_test_suite!(
            EchoTestSet,
            echo_anything,
            echo_anything,
            echo_anything,
            echo_anything
        );

        assert!(
            EchoTestSet::run(TestConfig::default().output(OutputStyle::None))
                .into_iter()
                .all(|test| test.test_result == TestResultType::Single(TestStatus::Success))
        );
    }
}
