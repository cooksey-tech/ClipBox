use std::{ffi::CStr, path::PathBuf};

use windows_sys::Win32::{Foundation::{GetLastError, HWND, POINT}, Graphics::Gdi::ScreenToClient, UI::WindowsAndMessaging::{ChildWindowFromPoint, ChildWindowFromPointEx, GetClassNameW, GetCursorPos, GetWindowLongPtrW, GWLP_USERDATA}};

use crate::tools::encoding::WideChar;



pub fn get_child_window(hwnd: HWND) -> HWND {
    let cursor_pos: *mut POINT = &mut POINT { x: 0, y: 0 };
    // Get the cursor position (x, y)
    match unsafe { GetCursorPos(cursor_pos) } {
        0 => {
            println!("Failed to get cursor position: {}", unsafe { GetLastError() });
            0
        }
        _ => {
            // Convert the cursor position to screen coordinates
            unsafe {
                ScreenToClient(hwnd, cursor_pos);
                return ChildWindowFromPoint(hwnd, *cursor_pos)
            }
        }
    }
}
