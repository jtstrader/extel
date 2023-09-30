//! Extel errors built using `thiserror`.

use std::{io, string::FromUtf8Error};
use thiserror::Error;

/// An Extel error type. Allows error propagation with [`ExtelResult`](crate::ExtelResult). Note
/// that this propagation does not panic the test runner but instead adjusts the test output to
/// reflect the error that is received.
///
/// # Example
/// ```rust
/// use std::fs;
/// use extel::prelude::*;
///
/// fn bad_file() -> ExtelResult {
///     let f = fs::File::open("./this_is_a_bad_file.txt")?;
///     pass!()
/// }
///
/// fn bad_test() -> ExtelResult {
///     extel_assert!(2 < 0, "test failed")
/// }
///
/// assert!(matches!(bad_file(), Err(Error::Io(_))));
/// assert!(matches!(bad_test(), Err(Error::TestFailed(_))));
/// ```
#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    TestFailed(String),
    #[error("an I/O error occurred")]
    Io(#[from] io::Error),
    #[error("invalid conversion from UTF-8 ocurred")]
    FromUtf8(#[from] FromUtf8Error),
}
