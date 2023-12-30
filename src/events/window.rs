use windows_sys::Win32::{
    UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId, GetClassNameW},
    Foundation::{HWND, HANDLE}, System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION}
};

pub fn foregroundWindow() {
    // Retrieves a handle to the foreground window
    // (the window with which the user is currently working).
    let handle: HWND;
    unsafe {
        handle = GetForegroundWindow();
    };
    println!("foregroundWindow: {:?}", handle);

    // Determine what application is running in the foreground.
    let process: HANDLE;

    unsafe {
        let mut process_id: u32 = 0;
        GetWindowThreadProcessId(handle, &mut process_id);
        println!("process_id: {:?}", process_id);
        process = OpenProcess(PROCESS_QUERY_INFORMATION, 0, process_id)
    };
    println!("foregroundWindow process: {:?}", process);

    // Find out what the app name is
    let mut name: [u16; 256] = [0; 256];
    unsafe {
        GetClassNameW(handle, &mut name as *mut u16, 256);
    }
    let name_string = String::from_utf16_lossy(&name);
    match name_string.trim_end_matches('\0') {
        "CabinetWClass" => println!("File Explorer"),
        _ => println!("Not File Explorer"),
    }

}
