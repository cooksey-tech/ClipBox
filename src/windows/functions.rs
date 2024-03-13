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
            println!("child_hwnd: {:?}", child_hwnd);

            if child_hwnd != 0 {

                // Check if the child window has a file path
                let classname = WideChar::from("");
                unsafe { GetClassNameW(child_hwnd, classname.as_ptr() as *mut u16, 256) };
                let class_string = unsafe { classname.to_string() };

                if class_string == "ICON_BOX" {
                    unsafe {
                        // We can access isize as u16 because we know that the pointer is a u16
                        let file_info = GetWindowLongPtrW(child_hwnd, GWLP_USERDATA);
                        println!("AFTER: {:?}", file_info);

                        let path_box = Box::from_raw(file_info as *mut WideChar);
                        let path = path_box.to_string();
                        // ensure that the memory is not deallocated
                        Box::leak(path_box);

                        println!("path_ptr(after): {:?}", path);

                        // let path = WideChar::from_ptr(*path_box as *const u16).to_string_lossy();

                        // println!("path: {:?}", path);

                        // let path_wide = WideChar::from_ptr(file_info as *const u16);


                        // let path_str = path_wide.to_string_lossy();


                    };
                }

            } else {
                println!("No child window found");
            }
        }
    }
}
