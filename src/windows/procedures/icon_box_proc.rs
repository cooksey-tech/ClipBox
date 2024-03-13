use windows_sys::Win32::{Foundation::{GetLastError, HWND, LPARAM, LRESULT, WPARAM}, Graphics::Gdi::{BeginPaint, EndPaint, HBRUSH, PAINTSTRUCT}, UI::WindowsAndMessaging::{DefWindowProcW, DrawIconEx, GetClassLongPtrW, GetClassNameW, GetClientRect, GetIconInfo, GetWindowLongPtrW, DI_NORMAL, GCLP_HICON, GWLP_USERDATA, HICON, WM_LBUTTONDOWN, WM_PAINT, WM_SETICON}};

use crate::{tools::encoding::WideChar, windows::functions::get_child_window};


// Define a window procedure
pub unsafe extern "system" fn icon_box_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    static mut HICON: Option<HICON> = None;

    match msg {
        WM_LBUTTONDOWN => {
            println!("Mouse click detected in icon_box");
            println!("icon_box_proc: {:?}", hwnd);
            // Get the child window under the cursor
            let child_hwnd = get_child_window(hwnd);

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



                    };
                }

            } else {
                println!("No child window found");
            }

            0
        }
        WM_SETICON => {
            println!("Setting icon in icon_box");
            HICON = Some(lparam as HICON);

            0
        }
        WM_PAINT => {
            println!("Painting icon_box");
            let mut ps: PAINTSTRUCT = unsafe { std::mem::zeroed() };
            let hdc = unsafe { BeginPaint(hwnd, &mut ps) };

            let hicon = HICON.expect("No icon found");

            // if let Some(hicon) = hicon {
                // get window dimensions
                let mut rect = unsafe { std::mem::zeroed() };
                unsafe { GetClientRect(hwnd, &mut rect) };

                // get icon dimensions
                let mut icon_info = unsafe { std::mem::zeroed() };
                unsafe { GetIconInfo(hicon, &mut icon_info) };
                let icon_w= icon_info.xHotspot as i32 * 3;
                let icon_h = icon_info.yHotspot as i32 * 3;

                let x = (rect.right - rect.left - icon_w) / 2;
                let y = (rect.bottom - rect.top - icon_h) / 2;

                // draw the icon
                let result = unsafe { DrawIconEx(hdc,
                    x,
                    y,
                    hicon,
                    // 60,
                    // 60,
                    icon_w,
                    icon_h,
                    0,
                    HBRUSH::default(),
                    DI_NORMAL)
                };
                if result == 0 {
                    let error = unsafe { GetLastError() };
                    println!("error: {:?}", error);
                }

            // }

            unsafe { EndPaint(hwnd, &ps) };
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
