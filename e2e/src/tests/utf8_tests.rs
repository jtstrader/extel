use extel::prelude::*;

pub(crate) fn good_utf8() -> ExtelResult {
    let utf8 = *b"\x00";
    let _ = String::from_utf8(utf8.into())?;
    pass!()
}

pub(crate) fn bad_utf8() -> ExtelResult {
    let utf8 = *b"\xFF";
    let _ = String::from_utf8(utf8.into())?;
    pass!()
}

pub(crate) fn original_handle_crash_way() -> ExtelResult {
    let utf8 = *b"\xFF";
    if String::from_utf8(utf8.into()).is_err() {
        return fail!("invalid conversion from UTF-8");
    }
    pass!()
}

init_test_suite!(
    Utf8TestSuite,
    good_utf8,
    bad_utf8,
    original_handle_crash_way
);
