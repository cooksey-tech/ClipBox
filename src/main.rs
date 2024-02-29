use std::{ptr::{null_mut, null}, os::raw::c_void, sync::{Arc, Mutex}};
use windows_sys::Win32::{
    UI::WindowsAndMessaging::{WM_INITMENUPOPUP, CreateWindowExW, ShowWindow, SW_SHOW, CreateWindowExA, WS_CHILD, WS_OVERLAPPEDWINDOW, CW_USEDEFAULT, RegisterClassA, WNDCLASSEXW, WNDCLASSA, WNDCLASSEXA, WNDPROC},
    System::DataExchange::{GetClipboardData, OpenClipboard, EmptyClipboard}, Foundation::{HWND, HANDLE},
    Foundation::{GetLastError, HINSTANCE}
};

use crate::{windows::window, storage::{clipbox::ClipBox, state::SharedState}};

mod events;
mod enums;
mod constants;
mod settings;
mod tools;
mod windows;
mod storage;
mod tests;


fn main() {
    // Sent when a drop-down menu or submenu is about to become active. This allows an application to modify the menu before it is displayed, without changing the entire menu.
    // match WM_INITMENUPOPUP {
    //     0x0117 => println!("WM_INITMENUPOPUP"),
    //     _ => println!("Not WM_INITMENUPOPUP"),
    // }

    events::mouse::lisenter();
    let (app, handle) = window::foreground_window();
    match (app, handle)  {
        (enums::app::App::FileExplorer, Some(_)) => println!("FileExplorer"),
        _ => println!("Unsupported"),
    }

    // create a test box
    let clip_box = ClipBox::new();

    // println!("clip_box: {:?}", state.clip_box.lock()
    //     .expect("Unable to block local thread").path);

    // let state_ptr = Arc::into_raw(state.clip_box.clone());
    // println!("state_ptr: {:?}", state_ptr);
    // println!("clip_box: {:?}", clip_box.path);

}
