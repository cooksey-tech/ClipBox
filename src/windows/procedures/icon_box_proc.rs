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
            SetCapture(hwnd);
            GetWindowRect(hwnd, &mut WINDOW_RECT);
            GetCursorPos(&mut CURSOR_LOC);
            // ScreenToClient(hwnd, &mut CURSOR_LOC);

            println!("last error: {:?}", GetLastError());

            0
        }
        WM_LBUTTONUP => {
            println!("Entering WM_LBUTTONUP in icon_box");
            ReleaseCapture();

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
