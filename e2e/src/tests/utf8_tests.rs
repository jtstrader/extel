use extel::prelude::*;

pub(crate) fn good_utf8() -> ExtelResult {
    let utf8 = *b"\x00";
    let _ = String::from_utf8(utf8.into())?;
    pass!()
}

pub(crate) fn bad_utf8() -> ExtelResult {
    let utf8 = *b"\xFF";
    let _ = std::fs::File::open("./asodfijasdoifasd")?;
    let _ = String::from_utf8(utf8.into())?;
    pass!()
}

pub(crate) fn original_handle_crash_way() -> ExtelResult {
    let utf8 = *b"\xFF";
    if let Err(_) = String::from_utf8(utf8.into()) {
        return fail!("invalid conversion from UTF-8");
    }
    pass!()
}
