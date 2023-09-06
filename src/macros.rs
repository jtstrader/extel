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
                let mut writer: Box<dyn ::std::io::Write> = match cfg.output {
                    OutputStyle::Stdout => Box::new(::std::io::stdout()),
                    OutputStyle::File(file_name) => {
                        let file_handle = ::std::fs::File::create(file_name).expect("could not open output file");
                        Box::new(file_handle)
                    },
                    OutputStyle::Buffer(buffer) => Box::new(buffer)
                };

                // Begin running tests and logging to the desired writer
                test_set
                    .tests
                    .into_iter()
                    .enumerate()
                    .map(|(test_id, test)| {
                        let test_result = test.run_test();
                        output_test_result(&mut writer, &test_result, test_id + 1);
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
        TestStatus::Fail(format!("this test failed?"))
    }

    #[test]
    fn init_test_suite() {
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
