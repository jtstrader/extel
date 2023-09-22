use extel::{fail, parameters, pass, ExtelResult};

#[parameters(4_f64, 16_f64, 30_f64, 32_f64)]
pub fn perfect_sqrt(x: f64) -> ExtelResult {
    let sqrt = x.sqrt();
    if sqrt == sqrt.floor() {
        pass!()
    } else {
        fail!("{} is not a perfect square", x)
    }
}

fn __calculate(x: f64, y: f64) -> f64 {
    (x + y) / y
}

pub fn calculate() -> ExtelResult {
    let result = __calculate(10_f64, 2_f64);
    if result > 0_f64 {
        pass!()
    } else {
        fail!("value of __calculate(10, 2) <= 0: got {}", result)
    }
}
