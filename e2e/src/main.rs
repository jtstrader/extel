pub mod tests;

use extel::prelude::*;
use tests::{
    command_tests::CommandTestSuite, math_tests::MathTestSuite,
    unsupported_errors::UnsupportedErrorTestSuite, utf8_tests::Utf8TestSuite,
};

fn main() {
    run_tests::<MathTestSuite>();
    run_tests::<CommandTestSuite>();
    run_tests::<Utf8TestSuite>();
    run_tests::<UnsupportedErrorTestSuite>();
}

/// Run test set with default configuration.
fn run_tests<S: RunnableTestSet>() {
    S::run(TestConfig::default());
}
