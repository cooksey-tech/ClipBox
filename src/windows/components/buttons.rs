use std::ptr::null_mut;

use windows_sys::Win32::{Foundation::HWND, System::LibraryLoader::GetModuleHandleW, UI::WindowsAndMessaging::{CreateWindowExW, BS_DEFPUSHBUTTON, HMENU, WS_CHILD, WS_TABSTOP, WS_VISIBLE}};

use crate::{constants::ID_EXPAND_BUTTON, tools::encoding::wide_char};


pub fn expand_button(hwnd: HWND, pos: (i32, i32), width: i32, height: i32) {
    println!("Creating expand button");


    unsafe {
        CreateWindowExW(
            0,
            wide_char("BUTTON"), // Button class
            wide_char("Expand"),  // Button text
            WS_TABSTOP | WS_VISIBLE | WS_CHILD | BS_DEFPUSHBUTTON as u32,
            pos.0,
            pos.1,
            width,
            height,
            hwnd,
            ID_EXPAND_BUTTON as HMENU,
            GetModuleHandleW(null_mut()),
            null_mut(),
        );
    }
}
