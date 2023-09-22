pub mod tests;

use extel::{init_test_suite, RunnableTestSet, TestConfig};
use tests::math_tests::{calculate, perfect_sqrt};

use crate::tests::command_tests::echo;

fn main() {
    init_test_suite!(MathTestSuite, calculate, perfect_sqrt);
    init_test_suite!(EchoTestSuite, echo);

    MathTestSuite::run(TestConfig::default());
    EchoTestSuite::run(TestConfig::default());
}
