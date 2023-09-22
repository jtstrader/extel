use extel::{fail, pass, ExtelResult, TestResultType, TestStatus};
use extel_parameterized::parameters;

#[parameters((1, 1), (2, 3))]
fn check_sum_into_two(sum: (i32, i32)) -> ExtelResult {
    const EXPECTED: i32 = 2;

    let (a, b) = sum;
    match a + b {
        EXPECTED => pass!(),
        _ => fail!("invalid sum: expected {}, got {}", EXPECTED, a + b),
    }
}

#[test]
fn parameters_tuples() {
    assert_eq!(
        check_sum_into_two().get_test_result(),
        TestResultType::Parameterized(vec![
            TestStatus::Success,
            TestStatus::Fail("invalid sum: expected 2, got 5".to_string())
        ])
    );
}
