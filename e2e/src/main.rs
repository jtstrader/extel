use extel::{init_test_suite, parameters, RunnableTestSet, TestConfig, TestStatus};

fn main() {
    init_test_suite!(MathTestSuite, calculate);
    MathTestSuite::run(TestConfig::default());
}

#[parameters(4_f64, 16_f64, 30_f64)]
fn perfect_sqrt(x: f64) -> TestStatus {
    let sqrt = x.sqrt();
    if sqrt == sqrt.floor() {
        TestStatus::Success
    } else {
        TestStatus::Fail(format!("{} is not a perfect square", x))
    }
}

fn __calculate(x: f64, y: f64) -> f64 {
    (x + y) / y
}

fn calculate() -> TestStatus {
    let result = __calculate(10_f64, 2_f64);
    if result > 0_f64 {
        TestStatus::Success
    } else {
        TestStatus::Fail(format!("value of __calculate(10, 2) <= 0: got {}", result))
    }
}
