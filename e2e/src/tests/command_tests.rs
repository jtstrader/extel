use extel::{cmd, fail, parameters, pass, ExtelResult};

#[parameters("hello world", "viva las vegas", "extel's working!")]
pub(crate) fn echo(x: &str) -> ExtelResult {
    let mut echo_cmd = cmd!("echo -n \"{}\"", x);
    let output = echo_cmd.output().unwrap();
    let string_output = String::from_utf8_lossy(&output.stdout);

    // Verify echo works correctly.
    if string_output == x {
        pass!()
    } else {
        fail!("expected '{}', got '{}'", x, string_output)
    }
}
