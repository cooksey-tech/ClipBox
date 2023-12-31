use std::{ptr::{null_mut, null}, os::raw::c_void};
use windows_sys::Win32::{
    UI::WindowsAndMessaging::{WM_INITMENUPOPUP, CreateWindowExW, ShowWindow, SW_SHOW, CreateWindowExA, WS_CHILD, WS_OVERLAPPEDWINDOW, CW_USEDEFAULT},
    System::DataExchange::{GetClipboardData, OpenClipboard, EmptyClipboard}, Foundation::{HWND, HANDLE},
    Foundation::GetLastError
};
use windows::core::Error;

mod events;
mod enums;
mod constants;
mod storage;

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
            0,
            "STATIC".as_ptr() as *const u16,
            "ClipBox".as_ptr() as *const u16,
            WS_CHILD,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            0 as HWND,
            0 as HANDLE,
            0 as HANDLE,
            null(),
        );
        let error = GetLastError();
        println!("error: {:?}", error);

        ShowWindow(window, SW_SHOW);
        println!("window: {:?}", window);
    }

    // loop {
    //     println!("Enter a message (or 'exit' to quit):");

    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input).unwrap();

    //     if input.trim() == "exit" {
    //         break;
    //     }

    //     println!("You entered: {}", input);
    // }
    crate::storage::create_box_dir();
}
