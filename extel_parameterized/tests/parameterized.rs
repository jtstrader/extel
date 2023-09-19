use extel::TestStatus;
use extel_parameterized::parameters;

#[parameters((1, 1), (2, 3))]
fn check_sum_into_two(sum: (i32, i32)) -> TestStatus {
    const EXPECTED: i32 = 2;

    let (a, b) = sum;
    match a + b {
        EXPECTED => TestStatus::Success,
        _ => TestStatus::Fail(format!("invalid sum: expected {}, got {}", EXPECTED, a + b)),
    }
}

#[test]
fn parameters_tuples() {
    assert_eq!(
        check_sum_into_two(),
        vec![
            TestStatus::Success,
            TestStatus::Fail("invalid sum: expected 2, got 5".to_string())
        ]
    );
}
