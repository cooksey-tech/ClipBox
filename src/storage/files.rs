use std::{ptr::null_mut, sync::{Arc, Mutex, MutexGuard}, path::PathBuf};
use windows_sys::Win32::UI::Shell::{HDROP, DragQueryFileW, DragFinish};
use crate::storage::paths::ClipBox;

pub fn file_drop(hdrop: HDROP, clip_box: MutexGuard<ClipBox>) {
    let mut file_count = 0;
    // get number of files droped
    // 0xFFFFFFFF represents all files
    unsafe { file_count = DragQueryFileW(hdrop, 0xFFFFFFFF, null_mut(), 0) };

    for i in 0..file_count {
        let mut file_name: [u16; 256] = [0; 256];
        // get file name
        unsafe { DragQueryFileW(hdrop, i, &mut file_name as *mut u16, 256) };
        let file_lossy = String::from_utf16_lossy(&file_name);
        let file_name_string = file_lossy.trim_end_matches('\0');
        println!("file_name_string: {:?}", file_name_string);

        // copy file to box directory
        println!("clip_box: {:?}", clip_box.path);
        clip_box.add_file(&PathBuf::from(file_name_string));
    }
    // release memory allocated for HDROP
    unsafe { DragFinish(hdrop) };
}
