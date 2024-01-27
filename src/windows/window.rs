use std::borrow::Borrow;
use std::f32::consts::E;
use std::ffi::{OsStr, OsString};
use std::iter::once;
use std::ops::Deref;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use windows_sys::Win32::Foundation::{WPARAM, LPARAM, LRESULT};
use windows_sys::Win32::Graphics::Gdi::{HBRUSH, PAINTSTRUCT, BeginPaint, EndPaint, CreatePen, PS_SOLID, SelectObject, Ellipse, DeleteObject};
use windows_sys::Win32::System::LibraryLoader::{GetModuleHandleExW, GetModuleHandleW};
use windows_sys::Win32::UI::WindowsAndMessaging::{ChangeWindowMessageFilterEx, DefWindowProcW, DispatchMessageW, GetClientRect, GetMessageW, GetWindowLongPtrW, MessageBoxExW, PostQuitMessage, SetWindowLongPtrW, TranslateMessage, BS_DEFPUSHBUTTON, CREATESTRUCTW, GWLP_USERDATA, HCURSOR, HICON, HMENU, MSG, MSGFLT_ALLOW, WM_COPYDATA, WM_CREATE, WM_DESTROY, WM_DROPFILES, WM_PAINT, WS_CHILD, WS_EX_ACCEPTFILES, WS_EX_APPWINDOW, WS_TABSTOP, WS_VISIBLE};
use windows_sys::Win32::{
    Foundation::{GetLastError, HANDLE, HINSTANCE, HWND},
    System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION},
    UI::WindowsAndMessaging::{
        CreateWindowExW, GetClassNameW, GetForegroundWindow, GetWindowThreadProcessId,
        RegisterClassExW, ShowWindow, CW_USEDEFAULT, SW_SHOW, WNDCLASSEXW, WNDPROC,
        WS_OVERLAPPEDWINDOW,
    },
};
use windows_sys::Win32::UI::Shell::{DragAcceptFiles, HDROP, DragQueryFileW, DragFinish};

use crate::enums::app::App;
use crate::storage::files::file_drop;
use crate::storage::paths::ClipBox;
use crate::tools::encoding::wide_char;
// use crate::windows::procedure::{self, window_proc};

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

pub fn create_window(clip_box: &ClipBox) {
    println!("STARTING CREATE_WINDOW");

    // get the pointer to the clip_box
    // let arc_ptr = Arc::into_raw(Arc::new(Mutex::new(clip_box)));

    let clip_box_arc = Arc::new(Mutex::new(clip_box));
    println!("clip_box_arc: {:?}", clip_box_arc);

    let arc_ptr = Arc::into_raw(clip_box_arc);

    println!("arc_ptr: {:?}", arc_ptr);

    // println!("clip_box_ptr: {:?}", clip_box_ptr.to_owned());
    // let new_box = unsafe { Arc::from_raw(*clip_box_ptr) };
    // println!("new_box: {:?}", new_box);



    // Convert class_name to null-terminated wide string
    let class_name = wide_char("ClipBox");

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
        300,
        300,
        HWND::default(),
        HMENU::default(),
        wc.hInstance,
        arc_ptr as *const _,
    ) };

    // Ensure functionality when running as admin
    if window <= 0 {
        panic!("Failed to create window");
    } else {
        // Allow WM_DROPFILES messages
        let success = unsafe { ChangeWindowMessageFilterEx(window, WM_DROPFILES, MSGFLT_ALLOW, std::ptr::null_mut()) };
        if success == 0 {
            panic!("Failed to change message filter for WM_DROPFILES");
        }

        // Allow WM_COPYDATA messages
        let success = unsafe { ChangeWindowMessageFilterEx(window, WM_COPYDATA, MSGFLT_ALLOW, std::ptr::null_mut()) };
        if success == 0 {
            panic!("Failed to change message filter for WM_COPYDATA");
        }

        // Allow 0x0049 messages (WM_COPYGLOBALDATA)
        let success = unsafe { ChangeWindowMessageFilterEx(window, 0x0049, MSGFLT_ALLOW, std::ptr::null_mut()) };
        if success == 0 {
            panic!("Failed to change message filter for 0x0049");
        }
    }


    unsafe { DragAcceptFiles(window, true as i32) };

    // let hwndButton = unsafe {
    //     CreateWindowExW(
    //     0,
    //     wide_char("BUTTON"), // Button class
    //     null_mut(),  // Styles
    //     WS_TABSTOP | WS_VISIBLE | WS_CHILD | BS_DEFPUSHBUTTON as u32,
    //     10,
    //     10,
    //     100,
    //     100,
    //     window,
    //     HMENU::default(),
    //     wc.hInstance,
    //     null_mut(),
    // ) };

    // let _ = unsafe { SetWindowLongPtrW(window, GWLP_USERDATA, clip_box_ptr as isize) };

    let error = unsafe { GetLastError() };
    // println!("error: {:?}", error);
    unsafe { ShowWindow(window, SW_SHOW) };
    println!("window: {:?}", window);

    // Process Windows messages
    let mut msg: MSG = unsafe { std::mem::zeroed() };

    // let msgBox = unsafe { MessageBoxExW(
    //     HWND::default(),
    //     wide_char("Hello World!"),
    //     wide_char("text"),
    //     0,
    //     0) };

    // let clip_box_ptr = Arc::into_raw(Arc::new(Mutex::new(clip_box)));
    // println!("clip_box_ptr: {:?}", clip_box_ptr);

    // unsafe { Arc::from_raw(clip_box_ptr) };

    // let clip_box = unsafe { Arc::from_raw(clip_box_ptr) };
    // println!("clip_box: {:?}", clip_box.lock().unwrap().path);

    println!("last error: {:?}", unsafe { GetLastError() });
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

pub extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    // println!("Processing message: {}", msg);

    match msg {
        WM_CREATE => {
            // Handle window creation
            println!("WM_CREATE");
            let createstruct = unsafe {
                 &*(lparam as *const CREATESTRUCTW)
            };
            let arc_ptr = createstruct.lpCreateParams as *const Arc<Mutex<ClipBox>>;
            // let clip_box = unsafe {
            //     Arc::clone(&*arc_ptr)
            // };
            // println!("clip_box: {:?}", clip_box);

            if arc_ptr as usize % std::mem::align_of::<Arc<Mutex<ClipBox>>>() != 0 {
                panic!("arc_ptr is not properly aligned");
            }

            unsafe {
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, arc_ptr as isize);
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
        WM_DROPFILES => {
            println!("WM_DROPFILES");
            let hdrop = wparam as HDROP;
            let arc_ptr = unsafe {
                GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const Arc<Mutex<ClipBox>>
            };

            println!("arc_ptr: {:?}", arc_ptr);
            // let user_data = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) };
            // println!("User data: {:?}", user_data as *const Arc<Mutex<ClipBox>>);

            if arc_ptr as usize % std::mem::align_of::<Arc<Mutex<ClipBox>>>() != 0 {
                panic!("arc_ptr is not properly aligned");
            }
            // assert!(!arc_ptr.is_null(), "clip_box_ptr is null");
            // let clip_box = unsafe {
            //     // Arc::clone(&*arc_ptr)
            //     Arc::from_raw(arc_ptr as *const Mutex<ClipBox>)
            // };

            // let clip_box = unsafe {
            //     Arc::clone(&*arc_ptr)
            // };

            println!("CALLING FILE_DROP");

            file_drop(hdrop, arc_ptr);
            println!("COMPLETED FILE_DROP");
            // it's best to keep file_count directly above the for..in loop
            // otherwise, the optimizer could create issues
            let file_count = unsafe { DragQueryFileW(hdrop, 0xFFFFFFFF, null_mut(), 0) };
            for i in 0..file_count {
                let mut file_name: [u16; 256] = [0; 256];
                unsafe { DragQueryFileW(hdrop, i, &mut file_name as *mut u16, 256) };
                let file_lossy = String::from_utf16_lossy(&file_name);
                let file_name_string = file_lossy.trim_end_matches('\0');
                println!("file_name_string: {:?}", file_name_string);
            }
            unsafe { DragFinish(hdrop) };
            0
        }
        WM_PAINT => {
            // Handle window painting
            let mut ps: PAINTSTRUCT = unsafe { std::mem::zeroed() };
            let hdc = unsafe { BeginPaint(hwnd, &mut ps) };

            let pen = unsafe {
                CreatePen(PS_SOLID, 1, 0)
            };
            let old_pen = unsafe {
                SelectObject(hdc, pen)
            };

            // dimesions of button
            let mut rect = unsafe { std::mem::zeroed() };
            unsafe { GetClientRect(hwnd, &mut rect) };

            // draw a circle for the button
            unsafe { Ellipse(hdc, rect.left, rect.top, rect.right, rect.bottom) };

            // Clean up
            unsafe {
                SelectObject(hdc, old_pen);
                DeleteObject(pen);
                EndPaint(hwnd, &ps);
            }

            0
        }
        _ => {
            // Handle other messages or pass to default handler
            return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
        }
    }
}
