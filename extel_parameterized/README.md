# Extel Parameterized - Writing Parameterized Tests in Rust
Extel Parameterized, or just *parameterized*, is a proc macro crate that serves to offer a
`parameters` macro for converting single argument functions into a valid function that
`Extel` can interpret.

```rust
use extel::prelude::*;
use extel_parameterized::parameters;

fn single_test() -> ExtelResult {
    let mut my_cmd = cmd!("echo -n \"hello world\"");
    let output = my_cmd.output().unwrap();
    let string_output = String::from_utf8_lossy(&output.stdout);

    extel_assert!(
        string_output == *"hello world",
        "expected 'hello world', got '{}'"
        string_output
    )
}

#[parameters(1, 2, -2, 4)]
fn param_test(x: i32) -> ExtelResult {
    extel_assert!(x > 0, "{} <= 0", x)
}

fn main() {
    init_test_suite!(ExtelDemo, single_test, param_test);
    ExtelDemo::run(TestConfig::default());
}
