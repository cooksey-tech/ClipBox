use std::ptr::null_mut;

use windows_sys::Win32::{Foundation::HWND, System::LibraryLoader::GetModuleHandleW, UI::WindowsAndMessaging::{CreateWindowExW, BS_DEFPUSHBUTTON, HMENU, WS_CHILD, WS_TABSTOP, WS_VISIBLE}};

use crate::tools::encoding::wide_char;


pub fn expand_button(hwnd: HWND) {
    return unsafe {
        CreateWindowExW(
            0,
            wide_char("BUTTON"), // Button class
            wide_char("Expand"),  // Button text
            WS_TABSTOP | WS_VISIBLE | WS_CHILD | BS_DEFPUSHBUTTON as u32,
            10,
            10,
            100,
            100,
            hwnd,
            HMENU::default(),
            GetModuleHandleW(null_mut()),
            null_mut(),
        );
    }
}
