use std::{ fs, path::{ Path, PathBuf } };
use anyhow::{ Context, Result };

use crate::settings::appconfigmodels::AppConfig;
use super::uuidmodel::UniqueId;

pub fn initialize() -> Result<AppConfig> {
    let config_path = Path::new("C:\\ProgramData\\CPS\\appconfig.json");

    create_folder_if_not_exists(config_path.to_path_buf())?;
    let app_config = load_app_config(config_path.to_path_buf())?;
    Ok(app_config)
}

fn create_folder_if_not_exists(config_path: PathBuf) -> Result<()> {
    let parent_dir = config_path.parent().with_context(|| "Failed to fetch parent directory")?;

    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir).with_context(|| "Failed to create directory")?;
    }

    Ok(())
}

fn load_app_config(config_path: PathBuf) -> Result<AppConfig> {
    let mut app_config_root: AppConfig;
    if !config_path.exists() {
        app_config_root = AppConfig::default();
        let generated = UniqueId::default()?;
        app_config_root.add_device(generated.id);

        let json = serde_json
            ::to_string_pretty(&app_config_root)
            .with_context(|| "Failed to serialize config file")?;
        fs::write(config_path, json).with_context(|| "Unable to write file")?;
    } else {
        let json = fs::read_to_string(&config_path).with_context(|| "Failed to read file")?;
        app_config_root = serde_json
            ::from_str(&json)
            .with_context(|| "Failed to deserialize config file")?;

        println!("Config file already exists at {:?}", config_path);
    }

    Ok(app_config_root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_app_config_when_file_does_not_exist() {
        let config_path = Path::new("appconfig.json");
        let app_config = load_app_config(config_path.to_path_buf()).unwrap();
        assert_eq!(app_config.devices.len(), 1);
    }
}
