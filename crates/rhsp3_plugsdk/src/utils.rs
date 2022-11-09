use rhsp3_internal_common::{ensure_lit, errors::*};
use std::{
    borrow::Cow,
    ffi::{c_char, CStr, CString},
};

pub unsafe fn decode_char<'a>(param: *const c_char) -> Result<Cow<'a, str>> {
    let str = CStr::from_ptr(param);
    let (cow, _, is_error) = encoding_rs::SHIFT_JIS.decode(str.to_bytes());
    ensure_lit!(code: InvalidParameter, !is_error, "Error decoding Shift-JIS string.");
    Ok(cow)
}

pub unsafe fn encode_str(param: &str) -> Result<CString> {
    let (cow, _, is_error) = encoding_rs::SHIFT_JIS.encode(param);
    ensure_lit!(code: InvalidParameter, !is_error, "Error encoding Shift-JIS string.");
    Ok(CString::new(cow.to_vec())?)
}
