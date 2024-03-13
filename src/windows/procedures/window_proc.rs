

// extern "system" is telling the Rust compiler to use the "system" ABI (Application Binary Interface) for this function.

use std::{borrow::Borrow, ffi::OsStr, iter::once, os::windows::ffi::OsStrExt, path::PathBuf, ptr::null_mut, sync::{Arc, Mutex}};

use windows_sys::Win32::{Foundation::{GetLastError, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM}, Graphics::Gdi::{BeginPaint, CreateSolidBrush, EndPaint, InvalidateRect, UpdateWindow, HBRUSH, PAINTSTRUCT}, System::LibraryLoader::GetModuleHandleW, UI::{Shell::{DragFinish, DragQueryFileW, SHGetFileInfoW, HDROP, SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON}, WindowsAndMessaging::{CreateWindowExW, DefWindowProcW, DrawIconEx, GetClientRect, GetIconInfo, GetWindowLongPtrW, PostQuitMessage, RegisterClassExW, SetWindowLongPtrW, CREATESTRUCTW, CS_OWNDC, DI_NORMAL, GWLP_USERDATA, HCURSOR, HICON, HMENU, WM_COMMAND, WM_CREATE, WM_DESTROY, WM_DROPFILES, WM_LBUTTONDOWN, WM_PAINT, WNDCLASSEXW, WS_CHILD, WS_OVERLAPPEDWINDOW, WS_VISIBLE}}};

use crate::{constants::ID_EXPAND_BUTTON, storage::clipbox::ClipBox, tools::encoding::WideChar, windows::{components::buttons::expand_button, functions::get_child_window, procedures::icon_box_proc}};

pub unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {


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

                    let _expand_hwnd = unsafe {
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
                // convert file_path to a null-terminated wide string
                let file_path = WideChar::from(file_path.to_str().expect("Failed to convert to string"));

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

                    HICON = Some(shfi.hIcon);
                    InvalidateRect(hwnd, null_mut(), true as i32);
                    // Create a new icon_box

                    if HICON.is_some() {
                        let hicon = HICON.expect("Failed to get HICON");

                        let class_name = WideChar::from("ICON_BOX").as_ptr();
                        let icon_class = WNDCLASSEXW {
                            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                            style: CS_OWNDC,
                            lpfnWndProc: Some(icon_box_proc), // we don't need to handle any messages for this class
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

                        // get window dimensions
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
                        // create a box to hold the icon
                        let icon_box = unsafe {
                            CreateWindowExW(
                                0,
                                class_name,
                                WideChar::from("ICON").as_ptr(),
                                WS_VISIBLE | WS_CHILD,
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

                        let temp_ptr = file_path.as_ptr() as isize;
                        println!("BEFORE: {:?}", temp_ptr);
                        let path_wide = WideChar::from_ptr(temp_ptr as *const u16);
                        println!("path_wide: {:?}", path_wide);

                        let str_path = path_wide.to_string_lossy();
                        println!("icon_box path: {:?}", str_path);

                        println!("BEFORE HWND: {:?}", icon_box);

                        // let path_ptr = Box::into_raw(Box::new(file_path)) as isize;
                        // println!("path_ptr(before): {:?}", path_ptr);
                        let path_ptr = Box::into_raw(Box::new(file_path)) as isize;
                        // Send file/directory path to be attached to icon_box
                        SetWindowLongPtrW(icon_box, GWLP_USERDATA, path_ptr);
                    }

                    // Trigger a WM_PAINT message to redraw the window with the new icon
                    UpdateWindow(hwnd);

                }
            }
            unsafe { DragFinish(hdrop) };
            println!("WM_DROPFILES end");
            0
        }
        WM_LBUTTONDOWN => {
            println!("\nWM_LBUTTONDOWN");

            // Get the child window under the cursor
            get_child_window(hwnd);



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
            println!("hdc: {:?}", hdc);

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

                // draw the icon
                let result = unsafe { DrawIconEx(hdc,
                    x,
                    y,
                    hicon,
                    60,
                    60,
                    // icon_w,
                    // icon_h,
                    0,
                    HBRUSH::default(),
                    DI_NORMAL)
                };
                if result == 0 {
                    let error = unsafe { GetLastError() };
                    println!("error: {:?}", error);
                }

                // create a button to expand items
                let width = 80;
                let height = 20;
                let px = (rect.right - rect.left - width) / 2;
                let py = rect.bottom - (height + 10);
                expand_button(hwnd, (px, py), width, height);
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
