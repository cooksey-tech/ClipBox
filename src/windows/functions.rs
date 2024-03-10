use std::{ffi::CStr, path::PathBuf};

use windows_sys::Win32::{Foundation::{GetLastError, HWND, POINT}, Graphics::Gdi::ScreenToClient, UI::WindowsAndMessaging::{ChildWindowFromPoint, ChildWindowFromPointEx, GetClassNameW, GetCursorPos, GetWindowLongPtrW, GWLP_USERDATA}};

use crate::tools::encoding::WideChar;



pub fn get_child_window(hwnd: HWND){
    let cursor_pos: *mut POINT = &mut POINT { x: 0, y: 0 };
    // Get the cursor position (x, y)
    match unsafe { GetCursorPos(cursor_pos) } {
        0 => {
            println!("Failed to get cursor position");
        }
        _ => {
            // Convert the cursor position to client coordinates
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
                        // We can access isize as u16 because we know that the pointer is a u16
                        let file_info = GetWindowLongPtrW(child_hwnd, GWLP_USERDATA) as *const u16;
                        let error = GetLastError();
                        println!("error: {:?}", error);

                        println!("file_info: {:?}", file_info);

                        let path_wide = WideChar::from_ptr(file_info);
                        println!("path_wide: {:?}", path_wide);

                        let path_str = path_wide.to_string();

                        println!("path: {:?}", path_str);
                    };
                }

            } else {
                println!("No child window found");
            }
        }
    }
}
