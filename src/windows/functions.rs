use std::{ffi::CStr, path::PathBuf};

use windows_sys::Win32::{Foundation::{HWND, POINT}, Graphics::Gdi::ScreenToClient, UI::WindowsAndMessaging::{ChildWindowFromPoint, ChildWindowFromPointEx, GetClassNameW, GetCursorPos, GetWindowLongPtrW, GWLP_USERDATA}};

use crate::tools::encoding::WideChar;



pub fn get_child_window(hwnd: HWND) {
    let cursor_pos: *mut POINT = &mut POINT { x: 0, y: 0 };
    match unsafe { GetCursorPos(cursor_pos) } {
        0 => {
            println!("Failed to get cursor position");
        }
        _ => {
            //
            unsafe { ScreenToClient(hwnd, cursor_pos) };
            
            let child_hwnd = unsafe { ChildWindowFromPoint(hwnd, *cursor_pos) };
            println!("\nchild_hwnd: {:?}", child_hwnd);

            if child_hwnd != 0 {
                println!("child_hwnd: {:?}", child_hwnd);
                // Check if the child window has a file path
                let classname = WideChar::from("");
                unsafe { GetClassNameW(child_hwnd, classname.as_ptr() as *mut u16, 256) };
                let class_string = unsafe { classname.to_string() };
                println!("classname: {:?}", class_string);

                if class_string == "ICON_BOX" {
                    unsafe {
                        let file_info = GetWindowLongPtrW(child_hwnd, GWLP_USERDATA);
                        let path_str = CStr::from_ptr(file_info as *const i8);
                        let path = PathBuf::from(path_str.to_str().expect("Failed to convert to string"));
                        println!("path: {:?}", path);
                    };
                }

            } else {
                println!("No child window found");
            }
        }
    }
}
