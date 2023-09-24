# Extel - Extended Testing Library
Extel is a testing library that intends to help create scalable test suites with stateless
tests. Extel's primary purpose is to make writing tests fast, easy, and performant. A common
use case for Extel is when writing integration tests for external binaries/executables. Extel
comes with some macros for creating and parsing command results to streamline the process of
creating and running CLI commands.

Think of Extel like a scripting language for tests. There's no need for importing a set of
wacky modules, or writing weird redirections in a shell language to capture stdout. Instead,
you can use Extel to save the output or status results of the commands you want to run, and
then chose where your test output goes, too!

## Usage
Extel is intended to function on zero argument or single argument functions (the former being
called *single* tests and the latter *parameterized* tests). After creating your test function,
you can register it using the `init_test_suite` macro. This will scaffold a struct containing
pointers to the functions passed in. Calling the `run` function on the generated struct will
immediately go through all tests and collect the results into a vector for the user to parse
through if they so wish.

Note that all Extel test functions expect an `ExtelResult` in its return. Using another type
will cause the macros generating the test suite to fail, or will return some weird errors while
using the `parameters` proc macro.

```rust
use extel::prelude::*;
use extel_parameterized::parameters;

fn single_test() -> ExtelResult {
    let mut my_cmd = cmd!("echo -n \"hello world\"");
    let output = my_cmd.output().unwrap();
    let string_output = String::from_utf8_lossy(&output.stdout);

    extel_assert!(
        string_output == *"hello world",
        "expected 'hello world', got '{}'",
        string_output
    )
}

#[parameters(1, 2, -2, 4)]
fn param_test(x: i32) -> ExtelResult {
    extel_assert!(x >= 0, "{} < 0", x)
}

fn main() {
    init_test_suite!(ExtelDemo, single_test, param_test);
    ExtelDemo::run(TestConfig::default());
}
