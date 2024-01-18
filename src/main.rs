use std::{ptr::{null_mut, null}, os::raw::c_void, sync::{Arc, Mutex}};
use windows_sys::Win32::{
    UI::WindowsAndMessaging::{WM_INITMENUPOPUP, CreateWindowExW, ShowWindow, SW_SHOW, CreateWindowExA, WS_CHILD, WS_OVERLAPPEDWINDOW, CW_USEDEFAULT, RegisterClassA, WNDCLASSEXW, WNDCLASSA, WNDCLASSEXA, WNDPROC},
    System::DataExchange::{GetClipboardData, OpenClipboard, EmptyClipboard}, Foundation::{HWND, HANDLE},
    Foundation::{GetLastError, HINSTANCE}
};

use crate::{windows::window, storage::{paths::ClipBox, state::SharedState}};

mod events;
mod enums;
mod constants;
mod settings;
mod tools;
mod windows;
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
    let (app, handle) = window::foreground_window();
    match (app, handle)  {
        (enums::app::App::FileExplorer, Some(_)) => println!("FileExplorer"),
        _ => println!("Unsupported"),
    }

    // create a test box
    let clip_box = ClipBox::new();
    // create a shared state
    let state = SharedState::new(clip_box);
    println!("clip_box: {:?}", state.clip_box.lock()
        .expect("Unable to block local thread").path);

    // println!("clip_box: {:?}", clip_box.path);

}
