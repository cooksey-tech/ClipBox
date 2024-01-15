// Purpose: Encoding utilities.
use core::iter::once;
use std::{ffi::OsStr, os::windows::ffi::OsStrExt};

pub fn wide_char(text: &str) -> *const u16 {
    OsStr::new(text).encode_wide().chain(once(0)).collect::<Vec<_>>().as_ptr()
}
