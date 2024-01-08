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

// Function to create a window to display the application
pub fn create_window() {
    unsafe {
        // Convert class_name to null-terminated wide string
        let class_name_wide: Vec<u16> = OsStr::new("ClipBox").encode_wide().chain(once(0)).collect();

        // register class
        let mut wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32, // The size, in bytes, of this structure.
            style: 0, // The class style(s). This member can be any combination of the Class Styles.
            lpfnWndProc: Some(window_proc), // A pointer to the window procedure. You must use the CallWindowProc function to call the window procedure.
            cbClsExtra: 0, // The number of extra bytes to allocate following the window-class structure. The system initializes the bytes to zero.
            cbWndExtra: 0, 
            hInstance: GetModuleHandleW(null_mut()) as HINSTANCE, // A handle to the instance that contains the window procedure for the class.
            hIcon: HICON::default(), // A handle to the class icon. This member must be a handle to an icon resource. If this member is NULL, the system provides a default icon.
            hCursor: HCURSOR::default(), // A handle to the class cursor. This member must be a handle to a cursor resource. If this member is NULL, an application must explicitly set the cursor shape whenever the mouse moves into the application's window.
            hbrBackground: HBRUSH::default(), // A handle to the class background brush. This member can be a handle to the brush to be used for painting the background, or it can be a color value. A color value must be one of the following standard system colors (the value 1 must be added to the chosen color). If a color value is given, you must convert it to one of the following HBRUSH types: COLOR_ACTIVEBORDER, COLOR_ACTIVECAPTION, COLOR_APPWORKSPACE, COLOR_BACKGROUND, COLOR_BTNFACE, COLOR_BTNSHADOW, COLOR_BTNTEXT, COLOR_CAPTIONTEXT, COLOR_GRAYTEXT, COLOR_HIGHLIGHT, COLOR_HIGHLIGHTTEXT, COLOR_INACTIVEBORDER, COLOR_INACTIVECAPTION, COLOR_MENU, COLOR_MENUTEXT, COLOR_SCROLLBAR, COLOR_WINDOW, or COLOR_WINDOWFRAME. For example, the color value 0xFF00FF represents the red component value 0xFF, the green component value 0x00, and the blue component value 0xFF. These color component values are then combined to form the DWORD value 0x00FF00FF.
            lpszMenuName: null_mut(), // A pointer to a null-terminated character string that specifies the resource name of the class menu, as the name appears in the resource file. If you use an integer to identify the menu, use the MAKEINTRESOURCE macro. If this member is NULL, windows belonging to this class have no default menu.
            lpszClassName: class_name_wide.as_ptr(), // A pointer to a null-terminated string or is an atom. If this parameter is an atom, it must be a class atom created by a previous call to the RegisterClass or RegisterClassEx function. The atom must be in the low-order word of lpszClassName; the high-order word must be zero.
            hIconSm: HICON::default(), // A handle to a small icon that is associated with the window class. If this member is NULL, the system searches the icon resource specified by the hIcon member for an icon of the appropriate size to use as the small icon.
        };

        let result = RegisterClassExW(&mut wc);

        let window = CreateWindowExW(
            0, // The extended window style of the window being created.
            class_name_wide.as_ptr(), // A null-terminated string or a class atom created by a previous call to the RegisterClass or RegisterClassEx function.
            class_name_wide.as_ptr(), // The window name. If the window style specifies a title bar, the window title pointed to by lpWindowName is displayed in the title bar. 
            WS_OVERLAPPEDWINDOW, // The style of the window being created.
            CW_USEDEFAULT, // The initial horizontal position of the window. For an overlapped or pop-up window, the x parameter is the initial x-coordinate of the window's upper-left corner, in screen coordinates. For a child window, x is the x-coordinate of the upper-left corner of the window relative to the upper-left corner of the parent window's client area.
            CW_USEDEFAULT, // The initial vertical position of the window. For an overlapped or pop-up window, the y parameter is the initial y-coordinate of the window's upper-left corner, in screen coordinates. For a child window, y is the initial y-coordinate of the upper-left corner of the child window relative to the upper-left corner of the parent window's client area.
            CW_USEDEFAULT, // The width, in device units, of the window. For overlapped windows, nWidth is the window's width, in screen coordinates, or CW_USEDEFAULT. If nWidth is CW_USEDEFAULT, the system selects a default width and height for the window; the default width extends from the initial x-coordinates to the right edge of the screen; the default height extends from the initial y-coordinate to the top of the icon area. CW_USEDEFAULT is valid only for overlapped windows; if CW_USEDEFAULT is specified for a pop-up or child window, the nWidth and nHeight parameters are set to zero.
            CW_USEDEFAULT, // The height, in device units, of the window. For overlapped windows, nHeight is the window's height, in screen coordinates. If nWidth is CW_USEDEFAULT, the system ignores nHeight.
            HWND::default(), // The parent or owner window of the window being created. To create a child window or an owned window, supply a valid window handle. This parameter is optional for pop-up windows.
            HMENU::default(), // The menu for the window being created. This parameter is optional and can be NULL.
            wc.hInstance, // A handle to the instance of the module to be associated with the window.
            null_mut(), // The lpParam parameter value to be passed to the window through the CREATESTRUCT structure (lpCreateParams member) pointed to by the lParam parameter. This message is sent to the created window by this function before it returns.
        );

        let error = GetLastError(); // Retrieves the calling thread's last-error code value. The last-error code is maintained on a per-thread basis. Multiple threads do not overwrite each other's last-error code.
        println!("error: {:?}", error);

        
        let window =ShowWindow(window, SW_SHOW); // Activates the window and displays it in its current size and position.
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
