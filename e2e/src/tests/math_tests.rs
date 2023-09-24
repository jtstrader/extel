use extel::prelude::*;

#[parameters(4_f64, 16_f64, 30_f64, 32_f64)]
pub fn perfect_sqrt(x: f64) -> ExtelResult {
    let sqrt = x.sqrt();
    extel_assert!(sqrt == sqrt.floor(), "{} is not a perfect square", x)
}

fn __calculate(x: f64, y: f64) -> f64 {
    (x + y) / y
}

pub fn calculate() -> ExtelResult {
    let result = __calculate(10_f64, 2_f64);
    extel_assert!(
        result > 0_f64,
        "value of __calculate(10, 2) <= 0: got {}",
        result
    )
}
