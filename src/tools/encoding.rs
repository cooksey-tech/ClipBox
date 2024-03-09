// Purpose: Encoding utilities.
use core::iter::once;
use std::{ffi::OsStr, os::windows::ffi::OsStrExt};

pub struct WideChar {
    data: Vec<u16>,
    ptr: *const u16,
}

impl From<&str> for WideChar {
    fn from(text: &str) -> Self {
        let data = OsStr::new(text).encode_wide().chain(once(0)).collect::<Vec<_>>();
        let ptr = data.as_ptr();
        WideChar { data, ptr }
    }
}

impl WideChar {
    pub fn as_ptr(&self) -> *mut u16 {
        self.ptr as *mut u16
    }
}

pub fn wide_char(text: &str) -> *const u16 {
    OsStr::new(text).encode_wide().chain(once(0)).collect::<Vec<_>>().as_ptr()
}


/// Determines the length of a wide character string.
/// A valid pionter must be passed to this function.
pub unsafe fn wcslen(s: *const u16) -> usize {
    let mut p = s;
    while *p != 0 {
        p = p.offset(1);
    }
    p.offset_from(s) as usize
}
