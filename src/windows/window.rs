use std::f32::consts::E;
use std::ffi::{OsStr, OsString};
use std::iter::once;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::ptr::null_mut;
use windows_sys::Win32::Foundation::{WPARAM, LPARAM, LRESULT};
use windows_sys::Win32::Graphics::Gdi::{HBRUSH, PAINTSTRUCT, BeginPaint, EndPaint, CreatePen, PS_SOLID, SelectObject, Ellipse, DeleteObject};
use windows_sys::Win32::System::LibraryLoader::{GetModuleHandleExW, GetModuleHandleW};
use windows_sys::Win32::UI::WindowsAndMessaging::{HICON, HCURSOR, HMENU, PostQuitMessage, DefWindowProcW, WM_PAINT, WM_DESTROY, MSG, GetMessageW, TranslateMessage, DispatchMessageW, WS_EX_APPWINDOW, WS_EX_ACCEPTFILES, WS_CHILD, WS_TABSTOP, WS_VISIBLE, BS_DEFPUSHBUTTON, MessageBoxExW, GetClientRect, WM_DROPFILES};
use windows_sys::Win32::{
    Foundation::{GetLastError, HANDLE, HINSTANCE, HWND},
    System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION},
    UI::WindowsAndMessaging::{
        CreateWindowExW, GetClassNameW, GetForegroundWindow, GetWindowThreadProcessId,
        RegisterClassExW, ShowWindow, CW_USEDEFAULT, SW_SHOW, WNDCLASSEXW, WNDPROC,
        WS_OVERLAPPEDWINDOW,
    },
};
use windows_sys::Win32::UI::Shell::{DragAcceptFiles, HDROP, DragQueryFileW, DragFinish};

use crate::enums::app::App;
use crate::storage::files::file_drop;
use crate::tools::encoding::wide_char;
// use crate::windows::procedure::{self, window_proc};

pub fn foreground_window() -> (App, Option<HWND>) {
    // Retrieves a handle to the foreground window
    // (the window with which the user is currently working).
    let handle: HWND;
    unsafe {
        handle = GetForegroundWindow();
    };
    println!("foregroundWindow: {:?}", handle);

    // Determine what application is running in the foreground.
    unsafe {
        let mut process_id: u32 = 0;
        GetWindowThreadProcessId(handle, &mut process_id);
        println!("process_id: {:?}", process_id);
        OpenProcess(PROCESS_QUERY_INFORMATION, 0, process_id)
    };

    // Find out what the app name is
    let mut name: [u16; 256] = [0; 256];
    unsafe {
        GetClassNameW(handle, &mut name as *mut u16, 256);
    }
    let name_string = String::from_utf16_lossy(&name);
    match name_string.trim_end_matches('\0') {
        "CabinetWClass" => (App::FileExplorer, Some(handle)),
        _ => (App::Unsupported, None),
    }
}

pub fn create_window() {

    // Convert class_name to null-terminated wide string
    let class_name = wide_char("ClipBox");

    // register class
    let mut wc = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: 0,
        lpfnWndProc: Some(self::window_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: unsafe { GetModuleHandleW(null_mut()) } as HINSTANCE,
        hIcon: HICON::default(),
        hCursor: HCURSOR::default(),
        hbrBackground: HBRUSH::default(),
        lpszMenuName: null_mut(),
        lpszClassName: class_name,
        hIconSm: HICON::default(),
    };
    unsafe { RegisterClassExW(&mut wc) };

    let window = unsafe { CreateWindowExW(
        0,
        class_name,
        class_name,
        WS_EX_ACCEPTFILES, // Window to accept drag and drop
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        300,
        300,
        HWND::default(),
        HMENU::default(),
        wc.hInstance,
        null_mut(),
    ) };
    unsafe { DragAcceptFiles(window, true as i32) };

    // let hwndButton = unsafe {
    //     CreateWindowExW(
    //     0,
    //     wide_char("BUTTON"), // Button class
    //     null_mut(),  // Styles
    //     WS_TABSTOP | WS_VISIBLE | WS_CHILD | BS_DEFPUSHBUTTON as u32,
    //     10,
    //     10,
    //     100,
    //     100,
    //     window,
    //     HMENU::default(),
    //     wc.hInstance,
    //     null_mut(),
    // ) };

    let error = unsafe { GetLastError() };
    // println!("error: {:?}", error);

    unsafe { ShowWindow(window, SW_SHOW) };
    println!("window: {:?}", window);

    // Process Windows messages
    let mut msg: MSG = unsafe { std::mem::zeroed() };

    // let msgBox = unsafe { MessageBoxExW(
    //     HWND::default(),
    //     wide_char("Hello World!"),
    //     wide_char("text"),
    //     0,
    //     0) };

    println!("last error: {:?}", unsafe { GetLastError() });
    unsafe {
        loop {
            match GetMessageW(&mut msg, window, 0, 0) {
                0 => {
                    println!("error 0: {:?}", error);
                    break
                },
                -1 => {
                    // Handle errors
                    println!("error -1: {:?}", GetLastError());
                    break;
                }
                _ => {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }

            }
        }
    }
}

pub extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_DESTROY => {
            // Handle window destruction
            unsafe { PostQuitMessage(0) };
            return 0;
        }
        WM_DROPFILES => {
            let hdrop = wparam as HDROP;
            println!("WM_DROPFILES: {:?}", hdrop);
            file_drop(hdrop);
            hdrop
        }
        WM_PAINT => {
            // Handle window painting
            let mut ps: PAINTSTRUCT = unsafe { std::mem::zeroed() };
            let hdc = unsafe { BeginPaint(hwnd, &mut ps) };

            let pen = unsafe {
                CreatePen(PS_SOLID, 1, 0)
            };
            let old_pen = unsafe {
                SelectObject(hdc, pen)
            };

            // dimesions of button
            let mut rect = unsafe { std::mem::zeroed() };
            unsafe { GetClientRect(hwnd, &mut rect) };

            // draw a circle for the button
            unsafe { Ellipse(hdc, rect.left, rect.top, rect.right, rect.bottom) };

            // Clean up
            unsafe {
                SelectObject(hdc, old_pen);
                DeleteObject(pen);
                EndPaint(hwnd, &ps);
            }

            return 0;
        }
        _ => {
            // Handle other messages or pass to default handler
            return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
        }
    }
}
