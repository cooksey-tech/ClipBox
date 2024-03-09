// Purpose: Encoding utilities.
use std::{ffi::OsStr, iter::once, os::windows::ffi::OsStrExt};

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
    pub fn as_ptr(&self) -> *const u16 {
        self.ptr
    }
    //#[cfg(debug_assertions)] // should not be a need to use this in release
    pub unsafe fn to_string(&self) -> String {
        use std::{ffi::OsString, os::windows::ffi::OsStringExt};


        let len = WideChar::wcslen(self.ptr);
        let slice = unsafe { std::slice::from_raw_parts(self.ptr, len) };

        OsString::from_wide(slice).into_string().expect("Failed to convert to string")
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
