use std::{ptr::{null_mut, null}, os::raw::c_void};
use windows_sys::Win32::{
    UI::WindowsAndMessaging::{WM_INITMENUPOPUP, CreateWindowExW, ShowWindow, SW_SHOW, CreateWindowExA, WS_CHILD, WS_OVERLAPPEDWINDOW, CW_USEDEFAULT, RegisterClassA, WNDCLASSEXW, WNDCLASSA, WNDCLASSEXA, WNDPROC},
    System::DataExchange::{GetClipboardData, OpenClipboard, EmptyClipboard}, Foundation::{HWND, HANDLE},
    Foundation::{GetLastError, HINSTANCE}
};

mod events;
mod enums;
mod constants;
mod storage;
pub mod settings;

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

    crate::storage::create_box_dir();
    events::window::create_window();

    
}
