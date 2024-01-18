use std::{path::PathBuf, env};

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
        // Copies the file to the box directory
        std::fs::copy(file_path, &self.path.join(file_path.file_name().unwrap()))
            .expect("Failed to copy file to box directory");
    }

    pub fn delete(self) {
        // Deletes the box directory
        std::fs::remove_dir_all(&self.path)
            .expect("Failed to delete box directory");
    }

    fn create_window(&self) {
        window::create_window(&self);
    }
}
