use std::{ptr::null_mut, thread};

use windows_sys::Win32::{Foundation::{GetLastError, HWND, LPARAM, LRESULT, POINT, RECT, TRUE, WPARAM}, Graphics::Gdi::{BeginPaint, EndPaint, InvalidateRect, ScreenToClient, HBRUSH, PAINTSTRUCT}, UI::{Input::KeyboardAndMouse::ReleaseCapture, WindowsAndMessaging::{DefWindowProcW, DrawIconEx, GetClassLongPtrW, GetClassNameW, GetClientRect, GetCursorPos, GetIconInfo, GetWindowLongPtrW, GetWindowRect, MoveWindow, SetWindowLongPtrW, SetWindowPos, DI_NORMAL, GCLP_HICON, GWLP_USERDATA, GWL_STYLE, HICON, HWND_TOP, MKF_LEFTBUTTONDOWN, SWP_NOSIZE, SWP_NOZORDER, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEMOVE, WM_NCHITTEST, WM_PAINT, WM_SETICON, WS_CHILD, WS_POPUP}}};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::SetCapture;
use windows_sys::Win32::System::SystemServices::MK_LBUTTON;

use crate::{tools::encoding::WideChar, windows::functions::get_child_window};


// Define a window procedure
pub unsafe extern "system" fn icon_box_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    static mut HICON: Option<HICON> = None;
    static mut CURSOR_LOC: POINT = POINT { x: 0, y: 0 };
    static mut LAST_LOC: POINT = POINT { x: 0, y: 0 };
    static mut WINDOW_RECT: RECT = RECT { left: 0, top: 0, right: 0, bottom: 0 };

    match msg {
        WM_LBUTTONDOWN => {
            println!("\nEntering WM_LBUTTONDOWN in icon_box - {:?}", hwnd);
            // SetWindowLongPtrW(hwnd, GWL_STYLE, WS_POPUP as _);


            SetCapture(hwnd);
            GetWindowRect(hwnd, &mut WINDOW_RECT);
            GetCursorPos(&mut CURSOR_LOC);
            // ScreenToClient(hwnd, &mut CURSOR_LOC);

            println!("last error: {:?}", GetLastError());
            // MOUSE_DOWN = true;



            // println!("icon_box_proc: {:?}", hwnd);

            // thread::spawn(move || {
            // Change to popup window
            // SetWindowLongPtrW(hwnd, GWLP_USERDATA, hwnd as _);

            // if hwnd != 0 {

            //     // Check if the child window has a file path
            //     let classname = WideChar::from("");
            //     unsafe { GetClassNameW(hwnd, classname.as_ptr() as *mut u16, 256) };
            //     let class_string = unsafe { classname.to_string() };

            //     if class_string == "ICON_BOX" {
            //         unsafe {
            //             // We can access isize as u16 because we know that the pointer is a u16
            //             let file_info = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
            //             println!("AFTER: {:?}", file_info);

            //             let path_box = Box::from_raw(file_info as *mut WideChar);
            //             let path = path_box.to_string();
            //             // ensure that the memory is not deallocated
            //             Box::leak(path_box);

            //             println!("path_ptr(after): {:?}", path);


            //             // get initial cursor location
            //             let cursor_pos = &mut POINT { x: 0, y: 0 };

            //             GetCursorPos(cursor_pos);
            //             // convert cursor location to screen coordinates
            //             ScreenToClient(hwnd, cursor_pos);
            //             CURSOR_LOC = *cursor_pos;

            //             // get initial icon_box window location
            //             let mut rect = std::mem::zeroed();
            //             GetWindowRect(hwnd, &mut rect);
            //             WINDOW_RECT = rect;
            //             WINDOW_LOC = POINT { x: rect.left, y: rect.top };
            //         };
            //     }

            // } else {
            //     println!("No child window found");
            // }
            // println!("Exiting WM_LBUTTONDOWN in icon_box - ${:?}", hwnd);
            // });

            0
        }
        WM_LBUTTONUP => {
            println!("Entering WM_LBUTTONUP in icon_box");
            ReleaseCapture();

            // Get the child window under the cursor
            // let child_hwnd = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as HWND;

            // Change to popup window
            // SetWindowLongPtrW(hwnd, GWL_STYLE, WS_CHILD as _);

            println!("Exiting WM_LBUTTONUP in icon_box - {:?}", hwnd);
            0
        }
        WM_MOUSEMOVE => {
            // println!("Mouse move detected in icon_box");

            GetCursorPos(&mut LAST_LOC);

            let width = WINDOW_RECT.right - WINDOW_RECT.left;
            let height = WINDOW_RECT.bottom - WINDOW_RECT.top;

            let x = LAST_LOC.x - CURSOR_LOC.x;
            let y = LAST_LOC.y - CURSOR_LOC.y;


            // If left mouse click is down
            if wparam as u32 == MK_LBUTTON {
                println!("x, y: {:?}, {:?}", x, y);
                let result = MoveWindow(hwnd, x, y, width, height, TRUE);
                if result == 0 {
                    let error = GetLastError();
                    println!("MoveWindow failed with error {}", error);
                } else {
                    println!("MoveWindow succeeded");
                }

                // // get the cursor location
                // let cursor_pos = &mut POINT { x: 0, y: 0 };
                // GetCursorPos(cursor_pos);
                // // ScreenToClient(hwnd, cursor_pos);

                // // update the icon_box window location
                // let x = WINDOW_LOC.x + (cursor_pos.x - CURSOR_LOC.x);
                // let y = WINDOW_LOC.y + (cursor_pos.y - CURSOR_LOC.y);
                // WINDOW_LOC = POINT { x, y };
                // // let mut client_point = POINT { x, y };
                // // ScreenToClient(hwnd, &mut client_point);
                // println!("x, y: ${:?}, ${:?}", x, y);

                // let width = WINDOW_RECT.right - WINDOW_RECT.left;
                // let height = WINDOW_RECT.bottom - WINDOW_RECT.top;
                // MoveWindow(hwnd, x, y, width, height, TRUE);

                // // InvalidateRect(hwnd, null_mut(), TRUE);
                // // SetWindowPos(hwnd, HWND_TOP, 100, 100, 0, 0, SWP_NOSIZE | SWP_NOZORDER);
                // // InvalidateRect(hwnd, null_mut(), TRUE);

                // CURSOR_LOC = *cursor_pos;
                // println!("window_loc: {:?}", WINDOW_LOC.x);

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
                // unsafe { GetClientRect(hwnd, &mut rect) };
                GetClientRect(hwnd, &mut rect);

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
