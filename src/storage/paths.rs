use std::{path::{PathBuf, Path}, env, fmt::Result};

use crate::constants;


// Returns the base path for the application
pub fn base_path() -> PathBuf {
    let appdata_path: String = env::var("LOCALAPPDATA").expect("LOCALAPPDATA environment variable not found");
    let app_data_dir: PathBuf = PathBuf::from(appdata_path).join(constants::APP_NAME);

    if !app_data_dir.exists() {
        std::fs::create_dir_all(&app_data_dir).expect("Failed to create application data directory");
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

        std::fs::create_dir(&base_path().join(&box_name)).expect("Failed to create box directory");

        ClipBox {
            path: base_path().join(box_name)
        }
    }

    pub fn delete(self) {
        // Deletes the box directory
        std::fs::remove_dir_all(&self.path).expect("Failed to delete box directory");
    }
}
