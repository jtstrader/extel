use extel::{prelude::*, TestResultType, TestStatus};
use extel_parameterized::parameters;

#[parameters((1, 1), (2, 3))]
fn check_sum_into_two(sum: (i32, i32)) -> ExtelResult {
    const EXPECTED: i32 = 2;
    let (a, b) = sum;
    extel_assert!(
        a + b == EXPECTED,
        "invalid sum: expected {}, got {}",
        EXPECTED,
        a + b
    )
}

#[parameters(vec![], vec![1])]
fn check_vec(x: Vec<usize>) -> ExtelResult {
    extel_assert!(!x.is_empty(), "input is empty")
}

#[parameters(1, 2, -1)]
pub fn check_pub_fn(x: i32) -> ExtelResult {
    extel_assert!(x >= 0, "x less than 0")
}

#[parameters(1, 2, -1)]
pub(crate) fn check_pub_crate_fn(x: i32) -> ExtelResult {
    extel_assert!(x >= 0, "x less than 0")
}

#[parameters(1, 2, -1)]
/// This is a doc comment.
fn doc_comment_fn(x: i32) -> ExtelResult {
    #[parameters(1)]
    fn y(x: i32) -> ExtelResult {
        pass!()
    }

    extel_assert!(x >= 0, "x less than 0")
}

mod super_test {
    use super::*;

    #[parameters(1, 2, -1)]
    pub(super) fn check_pub_super_fn(x: i32) -> ExtelResult {
        extel_assert!(x >= 0, "x less than 0")
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

#[test]
fn parameters_vec() {
    assert_eq!(
        check_vec().get_test_result(),
        TestResultType::Parameterized(vec![
            TestStatus::Fail("input is empty".to_string()),
            TestStatus::Success
        ])
    );
}

#[test]
fn parameters_pub() {
    assert_eq!(
        check_pub_fn().get_test_result(),
        TestResultType::Parameterized(vec![
            TestStatus::Success,
            TestStatus::Success,
            TestStatus::Fail("x less than 0".to_string())
        ])
    )
}

#[test]
fn parameters_pub_crate() {
    assert_eq!(
        check_pub_crate_fn().get_test_result(),
        TestResultType::Parameterized(vec![
            TestStatus::Success,
            TestStatus::Success,
            TestStatus::Fail("x less than 0".to_string())
        ])
    )
}

#[test]
fn parameters_pub_super() {
    use super_test::*;
    assert_eq!(
        check_pub_super_fn().get_test_result(),
        TestResultType::Parameterized(vec![
            TestStatus::Success,
            TestStatus::Success,
            TestStatus::Fail("x less than 0".to_string())
        ])
    )
}

#[test]
fn doc_comment() {
    assert_eq!(
        doc_comment_fn().get_test_result(),
        TestResultType::Parameterized(vec![
            TestStatus::Success,
            TestStatus::Success,
            TestStatus::Fail("x less than 0".to_string())
        ])
    )
}
