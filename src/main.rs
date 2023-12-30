use windows_sys::Win32::{
    UI::WindowsAndMessaging::{WM_INITMENUPOPUP},
    System::DataExchange::{GetClipboardData, OpenClipboard, EmptyClipboard},
};
mod events;

fn main() {
    println!("Hello, world!");

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
    events::window::foregroundWindow();
}
