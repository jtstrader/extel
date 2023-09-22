pub mod tests;

use extel::{init_test_suite, RunnableTestSet, TestConfig};
use tests::{command_tests::*, math_tests::*};

fn main() {
    init_test_suite!(MathTestSuite, calculate, perfect_sqrt);
    init_test_suite!(CommandTestSuite, echo, c_exe);

    MathTestSuite::run(TestConfig::default());
    CommandTestSuite::run(TestConfig::default());
}
