use std::{ptr::null_mut, sync::{Arc, Mutex, MutexGuard}, path::PathBuf};
use windows_sys::Win32::UI::Shell::{HDROP, DragQueryFileW, DragFinish};
use crate::storage::paths::ClipBox;

pub fn file_drop(hdrop: HDROP, clip_box: Arc<Mutex<ClipBox>>) {
    println!("file_drop");
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
        unsafe { DragQueryFileW(hdrop, i, file_name.as_mut_ptr(), file_name_len) };

        // convert file name to string
        let file_lossy = String::from_utf16_lossy(&file_name);
        let file_name_string = file_lossy.trim_end_matches('\0');
        println!("file_name_string: {:?}", file_name_string);
        // copy file to box directory
        clip_box_guard.add_file(&PathBuf::from(file_name_string));
        println!("completed add");
    }
    // release memory allocated for HDROP

    drop(clip_box_guard);
    unsafe { DragFinish(hdrop) };
}
