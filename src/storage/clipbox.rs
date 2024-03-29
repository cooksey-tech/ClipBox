use std::{env, fs, path::PathBuf, ptr::null_mut, thread};

use windows_sys::{core::HRESULT, Win32::{System::Com::FORMATETC, UI::Shell::{DragQueryFileW, HDROP}}};

use crate::{constants, windows::window};


// Returns the base path for the application
pub fn base_path() -> PathBuf {
    let appdata_path: String = env::var("LOCALAPPDATA")
        .expect("LOCALAPPDATA environment variable not found");
    let app_data_dir: PathBuf = PathBuf::from(appdata_path).join(constants::APP_NAME);

    if !app_data_dir.exists() {
        std::fs::create_dir_all(&app_data_dir)
            .expect("Failed to create application data directory");
    }

    app_data_dir
}


#[derive(Debug)]
pub struct ClipBox {
    pub path: PathBuf,
}

impl ClipBox {
    // Creates a new ClipBox instance
    pub fn new() -> ClipBox {
        // Creates a directory for a new box being created
        let timestamp = std::time::SystemTime::now();
        let box_name = format!("box_{}", timestamp.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());

        std::fs::create_dir(base_path().join(&box_name))
            .expect("Failed to create box directory");

        let clip_box = ClipBox {
            // location of the clipbox
            path: base_path().join(&box_name),
        };
        Self::create_window(&clip_box);

        clip_box
    }

    // Copy a file (or folder) to the box directory
    // Handles all files/folders dropped into the box
    pub fn file_drop(&self, hdrop: HDROP) {
        println!("STARTING FILE_DROP");

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

            let from_dir = PathBuf::from(file_name_string);
            let mut to_dir = self.path.clone();
            if from_dir.is_dir() {
                to_dir = self.path.join(from_dir.file_name()
                    .expect("Failed to get file name"));

                fs::create_dir_all(&to_dir)
                .expect("Failed to create directory");
            }
            self.copy_to(&to_dir, from_dir);
            println!("completed add");
        }
    }

    pub fn delete(self) {
        // Deletes the box directory
        std::fs::remove_dir_all(&self.path)
            .expect("Failed to delete box directory");
    }

    fn copy_to(&self, to_dir: &PathBuf, from_dir: PathBuf) {
        let folder_name = from_dir.file_name().expect("Failed to get file name");
        println!("\nfolder_name: {:?}", folder_name);

        // if a file, copy to box directory and return
        if from_dir.is_file() {
            println!("is file: {:?}", from_dir);
            println!("to_dir: {:?}", folder_name);

            std::fs::copy(&from_dir, to_dir.join(folder_name))
                .expect("Failed to copy file to box directory");
            return;
        }


        if from_dir.is_dir() { // if directory, create destination directory
            fs::create_dir_all(to_dir)
                .expect("Failed to create directory");
        }


        for file in from_dir.read_dir().expect("Failed to read directory") {
            // file is referring to the file we are currently iterating over
            // this will be the file/folder we are copying
            let file = file.expect("Failed to read file");
            let file_type = file.file_type().expect("Failed to get file type");
            let file_path = file.path();

            if file_type.is_dir() {
                let new_dir = to_dir.join(file.file_name());
                self.copy_to(&new_dir,file_path);
            } else {
                let file_name = file.file_name();
                let to_file = to_dir.join(file_name);

                println!("from: {:?}", file.path());
                println!("to: {:?}", to_file);

                // Copies the file to the box directory
                std::fs::copy(file_path, to_file)
                .expect("Failed to copy file to box directory");

            }
        }

    }


    fn create_window(&self) {
        window::create_window(self);
    }
}
