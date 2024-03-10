use std::borrow::Borrow;
use std::f32::consts::E;
use std::ffi::{CStr, OsStr, OsString};
use std::iter::once;
use std::ops::Deref;
use std::os::raw::c_void;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::PathBuf;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use windows_sys::Win32::System::Com::IDataObject;
use windows_sys::Win32::System::Ole::{DoDragDrop, OleInitialize, DROPEFFECT_COPY, DROPEFFECT_MOVE};
use windows_sys::Win32::Foundation::{LPARAM, LRESULT, POINT, WPARAM};
use windows_sys::Win32::Graphics::Gdi::{BeginPaint, CreatePen, CreateSolidBrush, DeleteObject, DrawCaption, Ellipse, EndPaint, InvalidateRect, SelectObject, UpdateWindow, COLOR_WINDOW, HBRUSH, HOLLOW_BRUSH, PAINTSTRUCT, PS_SOLID};
use windows_sys::Win32::System::LibraryLoader::{GetModuleHandleExW, GetModuleHandleW};
use windows_sys::Win32::UI::WindowsAndMessaging::{ChangeWindowMessageFilterEx, ChildWindowFromPoint, DefWindowProcW, DispatchMessageW, DrawIcon, DrawIconEx, GetClientRect, GetCursorPos, GetIconInfo, GetMessageW, GetWindowLongPtrW, MessageBoxExW, PostQuitMessage, SendMessageW, SetForegroundWindow, SetWindowLongPtrW, SetWindowPos, TranslateMessage, BS_DEFPUSHBUTTON, CREATESTRUCTW, CS_OWNDC, DI_NORMAL, GWLP_USERDATA, HCURSOR, HICON, HMENU, HWND_TOPMOST, MSG, MSGFLT_ALLOW, STM_SETICON, SWP_NOMOVE, SWP_NOSIZE, WM_COMMAND, WM_COPYDATA, WM_CREATE, WM_DESTROY, WM_DROPFILES, WM_LBUTTONDOWN, WM_PAINT, WS_CHILD, WS_EX_ACCEPTFILES, WS_EX_APPWINDOW, WS_TABSTOP, WS_VISIBLE};
use windows_sys::Win32::{
    Foundation::{GetLastError, HANDLE, HINSTANCE, HWND},
    System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION},
    UI::WindowsAndMessaging::{
        CreateWindowExW, GetClassNameW, GetForegroundWindow, GetWindowThreadProcessId,
        RegisterClassExW, ShowWindow, CW_USEDEFAULT, SW_SHOW, WNDCLASSEXW, WNDPROC,
        WS_OVERLAPPEDWINDOW,
    },
};
use windows_sys::Win32::UI::Shell::{DragAcceptFiles, DragFinish, DragQueryFileW, SHGetFileInfoW, HDROP, SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON};

use crate::constants::{ID_EXPAND_BUTTON, SS_ICON};
use crate::enums::app::App;
use crate::storage::clipbox::ClipBox;
use crate::tools::encoding::WideChar;
use crate::tools::{data_object::DataObject};
use crate::windows::functions::get_child_window;
use crate::windows::{components::buttons::expand_button};
use crate::windows::procedures::window_proc;

pub fn foreground_window() -> (App, Option<HWND>) {
    // Retrieves a handle to the foreground window
    // (the window with which the user is currently working).
    let handle: HWND;
    unsafe {
        handle = GetForegroundWindow();
    };
    println!("foregroundWindow: {:?}", handle);

    // Determine what applicatio is running in the foreground.
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

pub fn create_window(clip_box: &ClipBox) {

    // get the pointer to the clip_box
    let arc_ptr = Arc::into_raw(Arc::new(Mutex::new(clip_box)));
    let clip_box_ptr = Box::into_raw(Box::new(arc_ptr));
    println!("clip_box_ptr: {:?}", clip_box_ptr);

    // Convert class_name to null-terminated wide string
    let class_name = WideChar::from("ClipBox").as_ptr();

    // register class
    let mut wc = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: 0,
        lpfnWndProc: Some(window_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: unsafe { GetModuleHandleW(null_mut()) } as HINSTANCE,
        hIcon: HICON::default(),
        hCursor: HCURSOR::default(),
        hbrBackground: HBRUSH::default(),
        lpszMenuName: null_mut(),
        lpszClassName: class_name,
        hIconSm: HICON::default(),
    };
    unsafe { RegisterClassExW(&mut wc) };

    let window = unsafe { CreateWindowExW(
        0,
        class_name,
        class_name,
        WS_EX_ACCEPTFILES, // Window to accept drag and drop
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        160,
        160,
        HWND::default(),
        HMENU::default(),
        wc.hInstance,
        clip_box_ptr as *const std::os::raw::c_void, // Pass the clip_box_ptr as the lpParam
    ) };

    // Ensure functionality when running as admin
    if window <= 0 {
        panic!("Failed to create window");
    }
    let success = unsafe { SetWindowPos(window, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE) };
    if success == 0 {
            panic!("Failed to set window to topmost");
        }
    let success = unsafe { ChangeWindowMessageFilterEx(window, WM_DROPFILES, MSGFLT_ALLOW, std::ptr::null_mut()) };
    if success == 0 {
            panic!("Failed to change message filter for WM_DROPFILES");
        }
    let success = unsafe { ChangeWindowMessageFilterEx(window, WM_COPYDATA, MSGFLT_ALLOW, std::ptr::null_mut()) };
    if success == 0 {
            panic!("Failed to change message filter for WM_COPYDATA");
        }
    let success = unsafe { ChangeWindowMessageFilterEx(window, 0x0049, MSGFLT_ALLOW, std::ptr::null_mut()) };
    if success == 0 {
            panic!("Failed to change message filter for 0x0049");
        }


    unsafe {
        DragAcceptFiles(window, true as i32);
        ShowWindow(window, SW_SHOW);
    };

    unsafe {  };
    println!("window: {:?}", window);

    // Process Windows messages
    let mut msg: MSG = unsafe { std::mem::zeroed() };

    let error = unsafe { GetLastError() };
    println!("last error: {:?}", error);
    unsafe {
        loop {
            match GetMessageW(&mut msg, window, 0, 0) {
                0 => {
                    println!("error 0: {:?}", error);
                    break
                },
                -1 => {
                    // Handle errors
                    println!("error -1: {:?}", GetLastError());
                    break;
                }
                _ => {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }

            }
        }
    }
}

