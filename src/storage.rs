use std::{path::PathBuf, env};

use crate::constants;


pub fn storage() {
    let appdata_path = env::var("LOCALAPPDATA").expect("LOCALAPPDATA environment variable not found");
    let app_data_dir = PathBuf::from(appdata_path).join(constants::APP_NAME); // Replace "YourAppName" with your app's name

    println!("app_data_dir: {:?}", app_data_dir);
}
