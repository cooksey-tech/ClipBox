use std::{env, fs, path::PathBuf};

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
    pub fn add_file(&self, file_path: &PathBuf) {
        println!("STARTING ADD_FILE");

        let file_name = file_path.file_name().expect("Failed to get file name");
        println!("from: {:?}", file_name);
        println!("to: {:?}", &self.path.join(file_name));

        // Copies the file to the box directory
        std::fs::copy(file_path, &self.path.join(file_name))
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

        println!("to_dir: {:?}", to_dir);

        // create destination directory if it does not exist
        if !to_dir.is_dir() {
            fs::create_dir_all(to_dir)
                .expect("Failed to create directory");
        };

        for file in from_dir.read_dir().expect("Failed to read directory") {
            println!("inside of the iterator");
            let file = file.expect("Failed to read file");
            let file_type = file.file_type().expect("Failed to get file type");

            if file_type.is_dir() {
                println!("file is a directory");
                self.copy_to(&file.path());
            } else {
                let file_name = file.file_name();
                let to_file = to_dir.join(file_name);

                println!("from: {:?}", file.path());
                println!("to: {:?}", to_file);

                fs::copy(file.path(), to_file)
                    .expect("Failed to copy file");
            }


            // println!("file_type: {:?}", file_type);
        }
    }


    fn create_window(&self) {
        window::create_window(self);
    }
}
