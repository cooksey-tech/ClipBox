// Purpose: Encoding utilities.
use std::{ffi::OsStr, iter::once, os::windows::ffi::OsStrExt, path::PathBuf};

#[derive(Debug)]
pub struct WideChar {
    _wide: Vec<u16>,
    ptr: *const u16,
}
impl From<&str> for WideChar {
    fn from(text: &str) -> Self {
        let _wide: Vec<u16> = OsStr::new(&text).encode_wide().chain(once(0)).collect();
        let ptr = _wide.as_ptr();
        WideChar { _wide, ptr }
    }
}

impl WideChar {
    pub fn as_ptr(&self) -> *const u16 {
        self.ptr
    }
    pub unsafe fn from_ptr(ptr: *const u16) -> Self {
        let mut len = WideChar::wcslen(ptr);
        let _wide: Vec<u16> = std::slice::from_raw_parts(ptr, len as usize).to_owned();
        WideChar { _wide, ptr }
    }
    //#[cfg(debug_assertions)] // should not be a need to use this in release
    pub unsafe fn to_string(&self) -> String {
        use std::{ffi::OsString, os::windows::ffi::OsStringExt};


        let len = WideChar::wcslen(self.ptr);
        let slice = unsafe { std::slice::from_raw_parts(self.ptr, len) };

        OsString::from_wide(slice).into_string().expect("Failed to convert to string")
    }
    pub unsafe fn to_string_lossy(&self) -> String {
        let len = WideChar::wcslen(self.ptr);
        let slice = std::slice::from_raw_parts(self.ptr, len);
        String::from_utf16_lossy(slice)
    }

    /// Determines the length of a wide character string.
    /// A valid pionter must be passed to this function.
    unsafe fn wcslen(s: *const u16) -> usize {
        let mut p = s;
        while *p != 0 {
            p = p.offset(1);
        }
        p.offset_from(s) as usize
    }
}
