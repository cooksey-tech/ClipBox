use std::{ptr::{null_mut, null}, os::raw::c_void};
use windows_sys::Win32::{
    UI::WindowsAndMessaging::{WM_INITMENUPOPUP, CreateWindowExW, ShowWindow, SW_SHOW, CreateWindowExA, WS_CHILD, WS_OVERLAPPEDWINDOW, CW_USEDEFAULT, WS_EX_ACCEPTFILES, WS_EX_CLIENTEDGE, WS_EX_APPWINDOW},
    System::DataExchange::{GetClipboardData, OpenClipboard, EmptyClipboard}, Foundation::{HWND, HANDLE},
    Foundation::GetLastError
};
use windows::core::Error;

mod events;
mod enums;

fn main() {
    // Sent when a drop-down menu or submenu is about to become active. This allows an application to modify the menu before it is displayed, without changing the entire menu.
    match WM_INITMENUPOPUP {
        0x0117 => println!("WM_INITMENUPOPUP"),
        _ => println!("Not WM_INITMENUPOPUP"),
    }

    // Retrieves data from the clipboard in a specified format. The clipboard must have been opened previously.
    let result;
    unsafe {
        result = GetClipboardData(0);
    }
    println!("GetClipboardData: {:?}", result);

    let cb = unsafe {
        OpenClipboard(0)
    };
    unsafe {
        EmptyClipboard();
    }
    println!("OpenClipboard: {:?}", cb);

    events::mouse::lisenter();
    let (app, handle) = events::window::foreground_window();
    match (app, handle)  {
        (enums::app::App::FileExplorer, Some(_)) => println!("FileExplorer"),
        _ => println!("Unsupported"),
    }


    // Create a window
    unsafe {
        let window: HWND = CreateWindowExW(
            WS_EX_ACCEPTFILES, // Extended window style of the window being created
            "STATIC".as_ptr() as *const u16, // If string, specifies the window class name
            "ClipBox".as_ptr() as *const u16, // The window name
            WS_CHILD, // The style of the window being created
            CW_USEDEFAULT,  // The initial horizontal position of the window
            CW_USEDEFAULT, // The initial vertical position of the window
            CW_USEDEFAULT, // The width, in device units, of the window
            CW_USEDEFAULT, // The height, in device units, of the window
            0 as HWND, // A handle to the parent or owner window of the window being created
            0 as HANDLE, // A handle to a menu, or specifies a child-window identifier depending on the window style
            0 as HANDLE, // An application instance handle
            null(), // Pointer to window creation data
        );
        let error = GetLastError();
        println!("error: {:?}", error);

        ShowWindow(window, SW_SHOW);
        println!("window: {:?}", window);
    }

    loop {
        println!("Enter a message (or 'exit' to quit):");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        println!("You entered: {}", input);
    }
}
