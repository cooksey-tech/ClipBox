use std::{clone, path::PathBuf, ptr::null_mut, sync::{Arc, Mutex, MutexGuard}};
use windows_sys::Win32::UI::Shell::{HDROP, DragQueryFileW, DragFinish};
use crate::storage::paths::ClipBox;

pub fn file_drop(hdrop: HDROP, arc_ptr: *const Arc<Mutex<ClipBox>>) {
    println!("STARTING FILE_DROP");

    // println!("clip_box: {:?}", clip_box.to_owned());

    let clip_box = unsafe { Arc::from_raw(arc_ptr) };

    let clip_box_guard = match clip_box.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    // get number of files droped
    // 0xFFFFFFFF represents all files
    let file_count = unsafe { DragQueryFileW(hdrop, 0xFFFFFFFF, null_mut(), 0) };
    println!("file_count: {:?}", file_count);

    for i in 0..file_count {
        let file_name_len = unsafe { DragQueryFileW(hdrop, i, null_mut(), 0) } + 1;
        let mut file_name = vec![0u16; file_name_len as usize + 1];

        // get file nmae
        unsafe { DragQueryFileW(hdrop, i, file_name.as_mut_ptr(), file_name_len + 1) };

        // convert file name to string
        let file_name_string = match  String::from_utf16(&file_name) {
            Ok(s) => {
                s.trim_end_matches('\0').to_owned()
            },
            Err(_) => {
                println!("Failed to convert file name to string");
                continue;
            }
        };


        // let file_lossy = String::from_utf16_lossy(&file_name);
        // let file_name_string = file_lossy.trim_end_matches('\0');
        println!("file_name_string: {:?}", file_name_string);
        // copy file to box directory

        let file_path = PathBuf::from(file_name_string);
        println!("file_path: {:?}", file_path);

        clip_box_guard.add_file(&PathBuf::from(file_path));
        println!("completed add");
    }
    // release memory allocated for HDROP
    drop(clip_box_guard);
    unsafe { DragFinish(hdrop) };
}
