use std::{env, fs, path::PathBuf, thread};

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

    return app_data_dir;
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

        std::fs::create_dir(&base_path().join(&box_name))
            .expect("Failed to create box directory");

        let clip_box = ClipBox {
            path: base_path().join(box_name)
        };
        Self::create_window(&clip_box);

        clip_box
    }

    // Copy a file (or folder) to the box directory
    pub fn add_file(from_path: PathBuf, file_path: &PathBuf) {
        println!("STARTING ADD_FILE");

        let file_name = file_path.file_name().expect("Failed to get file name");
        println!("from: {:?}", file_name);
        println!("to: {:?}", &from_path.join(file_name));

        // Copies the file to the box directory
        std::fs::copy(file_path, &from_path.join(file_name))
            .expect("Failed to copy file to box directory");
    }

    pub fn delete(self) {
        // Deletes the box directory
        std::fs::remove_dir_all(&self.path)
            .expect("Failed to delete box directory");
    }

    pub fn copy_to(&self, from_dir: &PathBuf) {
        let folder_name = from_dir.file_name().expect("Failed to get file name");
        let to_dir = &self.path.join(folder_name);

        // if a file, copy to box directory and return
        if from_dir.is_file() {
            println!("is file: {:?}", from_dir);

            std::fs::copy(from_dir, to_dir)
                .expect("Failed to copy file to box directory");
            return;
        } else if !to_dir.is_dir() { // if directory, create destination directory if it does not exist
            fs::create_dir_all(to_dir)
                .expect("Failed to create directory");
        };

        // join handle for multiple threads
        let mut handles = Vec::new();


        for file in from_dir.read_dir().expect("Failed to read directory") {
            // file is referring to the file we are currently iterating over
            // this will be the file/folder we are copying
            let file = file.expect("Failed to read file");
            let file_type = file.file_type().expect("Failed to get file type");
            let file_path = file.path();

            if file_type.is_dir() {
                self.copy_to(&file_path);
            } else {
                let file_name = file.file_name();
                let to_file = to_dir.join(file_name);

                println!("from: {:?}", file.path());
                println!("to: {:?}", to_file);

                let handle = thread::spawn(move || {

                    // Copies the file to the box directory
                    std::fs::copy(file_path, to_file)
                    .expect("Failed to copy file to box directory");
                });

                handles.push(handle);
            }
        }

        for handle in handles {
            handle.join().expect("Failed to join thread");
        }

    }


    fn create_window(&self) {
        window::create_window(self);
    }
}
