use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use windows_sys::Win32::Foundation::{WPARAM, LPARAM, LRESULT};
use windows_sys::Win32::Graphics::Gdi::{HBRUSH, PAINTSTRUCT, BeginPaint, EndPaint};
use windows_sys::Win32::System::LibraryLoader::{GetModuleHandleExW, GetModuleHandleW};
use windows_sys::Win32::UI::WindowsAndMessaging::{HICON, HCURSOR, HMENU, PostQuitMessage, DefWindowProcW, WM_PAINT, WM_DESTROY};
use windows_sys::Win32::{
    Foundation::{GetLastError, HANDLE, HINSTANCE, HWND},
    System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION},
    UI::WindowsAndMessaging::{
        CreateWindowExW, GetClassNameW, GetForegroundWindow, GetWindowThreadProcessId,
        RegisterClassExW, ShowWindow, CW_USEDEFAULT, SW_SHOW, WNDCLASSEXW, WNDPROC,
        WS_OVERLAPPEDWINDOW,
    },
};

use crate::enums::app::App;

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
    unsafe {
        // Convert class_name to null-terminated wide string
        let class_name_wide: Vec<u16> = OsStr::new("ClipBox").encode_wide().chain(once(0)).collect();

        // register class
        let mut wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: 0,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: GetModuleHandleW(null_mut()) as HINSTANCE,
            hIcon: HICON::default(),
            hCursor: HCURSOR::default(),
            hbrBackground: HBRUSH::default(),
            lpszMenuName: null_mut(),
            lpszClassName: class_name_wide.as_ptr(),
            hIconSm: HICON::default(),
        };

        let result = RegisterClassExW(&mut wc);

        let window = CreateWindowExW(
            0,
            class_name_wide.as_ptr(),
            class_name_wide.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            HWND::default(),
            HMENU::default(),
            wc.hInstance,
            null_mut(),
        );

        let error = GetLastError();
        println!("error: {:?}", error);

        ShowWindow(window, SW_SHOW);
        println!("window: {:?}", window);
    }
}
extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_DESTROY => {
            // Handle window destruction
            unsafe { PostQuitMessage(0) };
            return 0;
        }
        WM_PAINT => {
            // Handle window painting
            let mut ps: PAINTSTRUCT = unsafe { std::mem::zeroed() };
            let hdc = unsafe { BeginPaint(hwnd, &mut ps) };
            // Perform painting operations using hdc
            unsafe { EndPaint(hwnd, &ps) };
            return 0;
        }
        _ => {
            // Handle other messages or pass to default handler
            return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
        }
    }
}
