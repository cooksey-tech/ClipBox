use std::{os::windows::ffi::OsStrExt, path::PathBuf};
use windows_sys::Win32::{Foundation::MAX_PATH, UI::{Shell::{SHGFI_ICON, SHGFI_LARGEICON}, WindowsAndMessaging::HICON}};
use windows_sys::Win32::UI::Shell::SHFILEINFOW;
use windows_sys::Win32::UI::Shell::SHGetFileInfoW;

use crate::tools::encoding::wide_char;

pub fn get_file_icon(path: &PathBuf) -> HICON {
    // Contains information about a file object.
    let mut shfi = SHFILEINFOW {
        hIcon: HICON::default(),
        iIcon: 0,
        dwAttributes: 0,
        szDisplayName: [0; MAX_PATH as usize],
        szTypeName: [0; 80],
    };

    unsafe {
        SHGetFileInfoW(
            wide_char(path.to_str().expect("Failed to convert path to string")),
            0,
            &mut shfi,
            std::mem::size_of::<SHFILEINFOW>() as u32,
            SHGFI_ICON | SHGFI_LARGEICON,
        );
    }

    shfi.hIcon
}

