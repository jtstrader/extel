pub mod tests;

use extel::prelude::*;
use tests::{command_tests::*, math_tests::*, utf8_tests::*};

fn main() {
    init_test_suite!(MathTestSuite, calculate, perfect_sqrt);
    init_test_suite!(CommandTestSuite, echo, c_exe);
    init_test_suite!(
        Utf8TestSuite,
        good_utf8,
        bad_utf8,
        original_handle_crash_way
    );

    MathTestSuite::run(TestConfig::default());
    CommandTestSuite::run(TestConfig::default());
    Utf8TestSuite::run(TestConfig::default());
}
