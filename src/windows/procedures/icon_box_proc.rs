use windows_sys::Win32::{Foundation::{HWND, LPARAM, LRESULT, WPARAM}, UI::WindowsAndMessaging::{DefWindowProcW, WM_LBUTTONDOWN}};

use crate::windows::functions::get_child_window;


// Define a window procedure
pub unsafe extern "system" fn icon_box_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {

    match msg {
        WM_LBUTTONDOWN => {
            println!("Mouse click detected in icon_box");

            // Get the child window at the cursor position
            get_child_window(hwnd);

            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
