use extel::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
enum UnsupportedError {
    #[allow(dead_code)]
    #[error("this is an error")]
    MyErrorVariant,
}

fn unsupported_error() -> ExtelResult {
    let foo = || -> Result<usize, UnsupportedError> { Ok(0) };

    // This would not compile!
    // let res = foo()?;

    // This will compile!
    let res = foo().map_err(|e| err!("{}", e))?;
    extel_assert!(res == 0)
}

init_test_suite!(UnsupportedErrorTestSuite, unsupported_error);
