use std::process::Stdio;

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

#[parameters(vec![], vec![1], vec![1, 2])]
pub(crate) fn c_exe(x: Vec<usize>) -> ExtelResult {
    let args = match x.is_empty() {
        true => String::new(),
        false => x
            .into_iter()
            .map(|u| u.to_string())
            .collect::<Vec<_>>()
            .join(" "),
    };

    let mut c_cmd = cmd!("./bin/test {}", args);
    let status = c_cmd.stdout(Stdio::null()).status().unwrap();

    // Verify that no errors occur
    if let Some(code) = status.code() {
        match code {
            0 => pass!(),
            _ => fail!("returned exit code: {}", code),
        }
    } else {
        fail!("could not extract exit code")
    }
}
