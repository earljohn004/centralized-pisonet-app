use std::{ fs, path::Path };

use crate::settings::models::AppConfig;

pub fn initialize() {
    let config_path = Path::new("C:\\ProgramData\\CPS\\appconfig.json");

    // Ensure the directory exists
    if let Some(parent_dir) = config_path.parent() {
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir).expect("Failed to create directory");
        }
    }

    if !config_path.exists() {
        let root = AppConfig::default();
        let json = serde_json::to_string_pretty(&root).unwrap();
        fs::write(config_path, json).expect("Unable to write file");
        println!("Initialized new config file at {:?}", config_path);
    } else {
        println!("Config file already exists at {:?}", config_path);
    }
}
