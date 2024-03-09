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
use windows_sys::Win32::UI::WindowsAndMessaging::{ChangeWindowMessageFilterEx, ChildWindowFromPoint, DefWindowProcW, DispatchMessageW, DrawIcon, DrawIconEx, GetClientRect, GetCursorPos, GetIconInfo, GetMessageW, GetWindowLongPtrW, MessageBoxExW, PostQuitMessage, SendMessageW, SetForegroundWindow, SetWindowLongPtrW, SetWindowPos, TranslateMessage, BS_DEFPUSHBUTTON, CREATESTRUCTW, DI_NORMAL, GWLP_USERDATA, HCURSOR, HICON, HMENU, HWND_TOPMOST, MSG, MSGFLT_ALLOW, STM_SETICON, SWP_NOMOVE, SWP_NOSIZE, WM_COMMAND, WM_COPYDATA, WM_CREATE, WM_DESTROY, WM_DROPFILES, WM_LBUTTONDOWN, WM_PAINT, WS_CHILD, WS_EX_ACCEPTFILES, WS_EX_APPWINDOW, WS_TABSTOP, WS_VISIBLE};
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
use crate::windows::components::buttons::expand_button;

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
        lpfnWndProc: Some(self::window_proc),
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

// extern "system" is telling the Rust compiler to use the "system" ABI (Application Binary Interface) for this function.
pub extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    // println!("Processing message: {}", msg);
    static mut HICON: Option<HICON> = None;
    static mut ICON_BOX: Option<HWND> = None;

    match msg {
        WM_CREATE => {
            // Handle window creation
            println!("WM_CREATE");
            let createstruct = unsafe {
                 &*(lparam as *const CREATESTRUCTW)
            };
            let box_ptr = createstruct.lpCreateParams as *mut *const Mutex<&ClipBox>;

            unsafe {
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, box_ptr as isize);
            }

            0
        }
        WM_DESTROY => {
            // Handle window destruction
            let clip_box_ptr = unsafe {
                GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Arc<Mutex<ClipBox>>
            };
            let _ = unsafe { Box::from_raw(clip_box_ptr) };  // Deallocate the Arc and the data
            unsafe { PostQuitMessage(0) };
            0
        }
        WM_COMMAND => {
            // Handle button press
            println!("WM_COMMAND");
            let button_id = wparam as i32;
            println!("button_id: {:?}", button_id);
            match button_id {
                ID_EXPAND_BUTTON => {
                    println!("Button 1 pressed");
                    let hinstance = unsafe { GetModuleHandleW(null_mut()) };

                    let expand_hwnd = unsafe {
                        CreateWindowExW(
                            0,
                            WideChar::from("EXPANDED_WINDOW").as_ptr(),
                            WideChar::from("Expanded Window").as_ptr(),
                            WS_OVERLAPPEDWINDOW,
                            100,
                            100,
                            100,
                            100,
                            hwnd,
                            HMENU::default(),
                            hinstance,
                            null_mut(),
                        )
                    };

                }
                _ => {
                    println!("Button not recognized");
                }
            }
            0
        }
        WM_DROPFILES => {
            println!("WM_DROPFILES");
            let hdrop = wparam as HDROP;
            let box_ptr = unsafe {
                GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut *const Mutex<&ClipBox>
            };
            let arc_ptr = unsafe {
                *Box::from_raw(box_ptr)
            };

            if arc_ptr as usize % std::mem::align_of::<Arc<Mutex<ClipBox>>>() != 0 {
                panic!("arc_ptr is not properly aligned");
            }
            assert!(!arc_ptr.is_null(), "clip_box_ptr is null");

            let arc = unsafe { Arc::from_raw(arc_ptr) };
            let arc_clone = Arc::clone(&arc);
            let clip_box_guard = arc_clone.lock().expect("Unable to unwrap Mutex"); // Lock the Mutex and keep the MutexGuard

            // convert the Arc to a raw pointer and transfer ownership to the Box
            let new_ptr = Box::into_raw(Box::new(Arc::into_raw(arc)));
            unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, new_ptr as isize) };

            clip_box_guard.file_drop(hdrop);
            // it's best to keep file_count directly above the for..in loop
            // otherwise, the optimizer could create issues
            let file_count = unsafe { DragQueryFileW(hdrop, 0xFFFFFFFF, null_mut(), 0) };
            for i in 0..file_count {
                let mut file_name: [u16; 256] = [0; 256];
                unsafe { DragQueryFileW(hdrop, i, &mut file_name as *mut u16, 256) };
                let file_lossy = String::from_utf16_lossy(&file_name);
                let file_name_string = PathBuf::from(&file_lossy.trim_end_matches('\0'));
                let file_path = clip_box_guard.path.join(file_name_string.file_name().expect("Failed to get file name"));
                println!("file_path: {:?}", file_path);

                // get file icon
                let mut shfi: SHFILEINFOW = unsafe { std::mem::zeroed() };
                let flags = SHGFI_ICON | SHGFI_LARGEICON;
                let file_path: Vec<u16> = OsStr::new(&file_path).encode_wide().chain(once(0)).collect();
                println!("file_path: {:?}", String::from_utf16_lossy(file_path.borrow()).trim_end_matches('\0'));
                let result = unsafe {
                    SHGetFileInfoW(
                        file_path.as_ptr(),
                        0,
                        &mut shfi as *mut _,
                        std::mem::size_of::<SHFILEINFOW>() as u32,
                        flags
                    )
                };
                if result != 0 {
                    unsafe {
                        HICON = Some(shfi.hIcon);
                        InvalidateRect(hwnd, null_mut(), true as i32);
                        // Send file/directory path to be attached to icon_box
                        SetWindowLongPtrW(hwnd, GWLP_USERDATA, file_path.as_ptr() as isize);
                        // Trigger a WM_PAINT message to redraw the window with the new icon
                        UpdateWindow(hwnd);
                    };
                }
            }
            unsafe { DragFinish(hdrop) };
            println!("WM_DROPFILES end");
            0
        }
        WM_LBUTTONDOWN => {
            println!("WM_LBUTTONDOWN");

            // Get the child window under the cursor
            let cursor_pos: *mut POINT = &mut POINT { x: 0, y: 0 };
            match unsafe { GetCursorPos(cursor_pos) } {
                0 => {
                    println!("Failed to get cursor position");
                }
                _ => {
                    unsafe { println!("cursor_pos: {:?}", (*cursor_pos).x) };
                    let child_hwnd = unsafe { ChildWindowFromPoint(hwnd, POINT { x: (*cursor_pos).x, y: (*cursor_pos).y }) };

                    if child_hwnd != 0 {
                        println!("child_hwnd: {:?}", child_hwnd);
                        // Check if the child window has a file path
                        let classname = WideChar::from("");
                        unsafe { GetClassNameW(child_hwnd, classname.as_ptr() as *mut u16, 256) };
                        let class_string = unsafe { classname.to_string() };
                        println!("classname: {:?}", class_string);

                        if class_string == "ICON_BOX" {
                            unsafe {
                                let file_info = GetWindowLongPtrW(child_hwnd, GWLP_USERDATA);
                                let path_str = CStr::from_ptr(file_info as *const i8);
                                let path = PathBuf::from(path_str.to_str().expect("Failed to convert to string"));
                                println!("path: {:?}", path);
                            };
                        }

                    } else {
                        println!("No child window found");
                    }
                }
            }


            // println!("child_hwnd: {:?}", child_hwnd);

            // DoDragDrop process starts here
            // unsafe { OleInitialize(null_mut()) };
            // this will contain the data to be dragged

            0
        }
        WM_PAINT => {
            println!("WM_PAINT");

            let mut ps: PAINTSTRUCT = unsafe { std::mem::zeroed() };
            let hdc = unsafe { BeginPaint(hwnd, &mut ps) };

            if let Some(hicon) = unsafe { HICON } {
                // get window dimensions
                let mut rect = unsafe { std::mem::zeroed() };
                unsafe { GetClientRect(hwnd, &mut rect) };

                // get icon dimensions
                let mut icon_info = unsafe { std::mem::zeroed() };
                unsafe { GetIconInfo(hicon, &mut icon_info) };
                let icon_w= icon_info.xHotspot as i32 * 2;
                let icon_h = icon_info.yHotspot as i32 * 2;

                let x = (rect.right - rect.left - icon_w) / 2;
                let y = (rect.bottom - rect.top - icon_h) / 2;

                if unsafe { ICON_BOX }.is_none()
                    || unsafe { ICON_BOX }.expect("Failed to get ICON_BOX") == HWND::default() {
                    // create icon_box class
                    let class_name = WideChar::from("ICON_BOX").as_ptr();
                    let icon_class = WNDCLASSEXW {
                        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                        style: 0,
                        lpfnWndProc: Some(DefWindowProcW), // we don't need to handle any messages for this class
                        cbClsExtra: 0,
                        cbWndExtra: 0,
                        hInstance: unsafe { GetModuleHandleW(null_mut()) } as HINSTANCE,
                        hIcon: hicon,
                        hCursor: HCURSOR::default(),
                        hbrBackground: unsafe { CreateSolidBrush(0x0000FF) }, // use the default window color
                        lpszMenuName: null_mut(),
                        lpszClassName: class_name,
                        hIconSm: hicon,
                    };
                    unsafe { RegisterClassExW(&icon_class) };

                    // create a box to hold the icon
                    let icon_box = unsafe {
                        CreateWindowExW(
                            0,
                            class_name,
                            WideChar::from("ICON").as_ptr(),
                            WS_VISIBLE | WS_CHILD | SS_ICON,
                            x,
                            y,
                            icon_w,
                            icon_h,
                            hwnd,
                            HMENU::default(),
                            icon_class.hInstance,
                            null_mut(),
                        )
                    };

                    unsafe { ICON_BOX = Some(icon_box) };

                // // set file path to icon_box
                // let file_path = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const u16 };
                // unsafe { SetWindowLongPtrW(icon_box, GWLP_USERDATA, file_path as isize) };

                // unsafe {
                //     SendMessageW(icon_box, STM_SETICON, hicon as usize, lparam);
                // }
                let icon_hnd = unsafe { BeginPaint(icon_box, &mut ps) };


                unsafe { DrawIconEx(hdc,
                    x,
                    y,
                    hicon,
                    icon_w,
                    icon_h,
                    0,
                    HBRUSH::default(),
                    DI_NORMAL)
                };

                // create a button to expand items
                let width = 80;
                let height = 20;
                let px = (rect.right - rect.left - width) / 2;
                let py = rect.bottom - (height + 10);
                expand_button(hwnd, (px, py), width, height);
                }
            }


            unsafe { EndPaint(hwnd, &ps) };

            0
        }
        _ => {
            // Handle other messages or pass to default handler
            unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
        }
    }
}
