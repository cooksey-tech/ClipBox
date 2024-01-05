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
        // register class
        let class_name = "ClipBox\0".as_ptr() as *const u16; // Ensure null-terminated string
        let mut wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32, // Size of structure (bytes)
            style: 0, // Determines attributes like window appearance and behavior
            lpfnWndProc: Some(window_proc), // Pointer to the window procedure function
            cbClsExtra: 0, // Number of extra bytes to allocate for class data
            cbWndExtra: 0, // Number of extra bytes to allocate for each window of this class
            hInstance: GetModuleHandleW(null_mut()) as HINSTANCE, // Handle to the instance of the module that contains the window procedure
            hIcon: HICON::default(), // Used to represent windows of this class in taskbars, title bars, etc
            hCursor: HCURSOR::default(), // Used to represent windows of this class in taskbars, title bars, etc
            hbrBackground: HBRUSH::default(), // Handle to the brush used to paint the background of windows of this class
            lpszMenuName: null_mut(), // Pointer to a null-terminated string that specifies the name of the class menu, if any
            lpszClassName: class_name, // Pointer to a null-terminated string that specifies the window class name
            hIconSm:HICON::default(), // Used in places like the taskbar or title bar when icons are displayed in smaller sizes
        };
        let result = RegisterClassExW(&mut wc);

        let window = CreateWindowExW(
            0, // Optional window styles
            wc.lpszClassName, // Pointer to a null-terminated string specifying the name of the window class
            "ClipBox\0".as_ptr() as *const u16, // Pointer to a null-terminated string that will be the window's title
            WS_OVERLAPPEDWINDOW, // Window style, including title bar, sizing border, window menu, and minimize/maximize buttons
            CW_USEDEFAULT, // Initial horizontal position
            CW_USEDEFAULT, // Initial vertical position
            CW_USEDEFAULT, // Initial width of the window
            CW_USEDEFAULT, // Initial height of the window
            HWND::default(), // Handle to the parent or owner window
            HMENU::default(), // Handle to a menu, or specifies a child-window identifier depending on the window style
            wc.hInstance, // Handle to the instance of the module that created the window
            null_mut(), // Pointer to a value to be passed to the window through the WM_CREATE message
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
