use std::collections::HashMap;

use anyhow::Context;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub version: String,
    #[serde(flatten)]
    pub devices: HashMap<String, DeviceConfig>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceConfig {
    pub config: InnerConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InnerConfig {
    pub client: HashMap<String, Client>,
    pub server: Server,
    pub license: License,
    pub ui: UserInterface,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Client {
    pub address: String,
    pub hwid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub address: String,
    pub port: String,
    pub hwid: String,
    pub password: String,
    pub configpath: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct License {
    pub authorized: bool,
    pub serial_number: String,
    pub email_address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInterface {
    pub cafe_name: String,
    pub station_id: String,
    pub insert_coin_text: String,
    pub autoshutdown_text: String,
    pub smwindow_position: String,
    pub background_img: String,
    pub countdown_timer: u8,
}

impl Default for UserInterface {
    fn default() -> Self {
        UserInterface {
            cafe_name: "MPG Cafe".to_string(),
            station_id: "station-01".to_string(),
            insert_coin_text: "Insert Coin".to_string(),
            autoshutdown_text: "Auto Shutdown in".to_string(),
            smwindow_position: "top-right".to_string(),
            background_img: "none".to_string(),
            countdown_timer: 100,
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Server {
            address: "127.0.0.1".to_string(),
            port: "3000".to_string(),
            hwid: "".to_string(),
            password: "".to_string(),
            configpath: "".to_string(),
        }
    }
}

impl Default for License {
    fn default() -> Self {
        License {
            authorized: false,
            serial_number: "".to_string(),
            email_address: "".to_string(),
        }
    }
}

impl License {
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "authorized": self.authorized,
            "serialNumber": self.serial_number,
            "emailAddress": self.email_address,
        })
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            version: "1.0".to_string(),
            devices: HashMap::new(),
        }
    }
}

impl AppConfig {
    pub fn add_device(&mut self, device_name: String) -> &mut Self {
        let default_device_config = DeviceConfig { config: InnerConfig::default() };
        self.devices.insert(device_name, default_device_config);
        self
    }

    pub fn get_ip_address(&self, device_name: &str) -> Result<String, anyhow::Error> {
        self.devices
            .get(device_name)
            .map(|device| device.config.server.address.clone())
            .with_context(|| "Ip address is not configured")
    }

    pub fn get_port(&self, device_name: &str) -> Result<String, anyhow::Error> {
        self.devices
            .get(device_name)
            .map(|device| device.config.server.port.clone())
            .with_context(|| "Port is configured")
    }

    pub fn get_license(&self, device_name: &str) -> Result<License, anyhow::Error> {
        self.devices
            .get(device_name)
            .map(|device| device.config.license.clone())
            .with_context(|| "License is not configured")
    }

    pub fn set_license(
        &mut self,
        device_name: &str,
        license: License
    ) -> Result<(), anyhow::Error> {
        if let Some(device) = self.devices.get_mut(device_name) {
            device.config.license = license;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Device configuration is not found!"))
        }
    }

    pub fn get_ui_config(
        &self,
        device_name: &str
    ) -> Result<UserInterface, anyhow::Error> {
        self.devices
            .get(device_name)
            .map(|device| device.config.ui.clone())
            .with_context(|| "UI configuration is not found")
    }

    pub fn set_ui_config(
        &mut self,
        device_name: &str,
        ui_config: UserInterface
    ) -> Result<(), anyhow::Error> {
        if let Some(device) = self.devices.get_mut(device_name) {
            device.config.ui = ui_config;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Device configuration is not found!"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_device() {
        let mut app_config = AppConfig::default();
        app_config.add_device("device1".to_string());
        assert_eq!(app_config.devices.len(), 1);
        app_config.add_device("device2".to_string());
        assert_eq!(app_config.devices.len(), 2);
    }

    #[test]
    fn test_length_is_one_when_adding_the_same_device() {
        let mut app_config = AppConfig::default();
        app_config.add_device("device1".to_string());
        assert_eq!(app_config.devices.len(), 1);
        app_config.add_device("device1".to_string());
        assert_eq!(app_config.devices.len(), 1);
    }

    #[test]
    fn test_get_ip_address_when_device_exists() {
        let mut app_config = AppConfig::default();
        app_config.add_device("device1".to_string());
        let ip_address = app_config.get_ip_address("device1").unwrap();
        assert_eq!(ip_address, "127.0.0.1");
    }

    #[test]
    fn test_get_ip_address_when_device_does_not_exist() {
        let app_config = AppConfig::default();
        let ip_address = app_config.get_ip_address("device1");
        assert!(ip_address.is_err());
    }

    #[test]
    fn test_get_port_when_device_exists() {
        let mut app_config = AppConfig::default();
        app_config.add_device("device1".to_string());
        let port = app_config.get_port("device1").unwrap();
        assert_eq!(port, "3000");
    }

    #[test]
    fn test_get_port_when_device_does_not_exist() {
        let app_config = AppConfig::default();
        let port = app_config.get_port("device1");
        assert!(port.is_err());
    }

    #[test]
    fn test_set_and_get_user_config() {
        let mut app_config = AppConfig::default();
        app_config.add_device("device1".to_string());
        let ui_config = UserInterface {
            cafe_name: "Test Cafe".to_string(),
            station_id: "station-01".to_string(),
            insert_coin_text: "Insert Coin".to_string(),
            autoshutdown_text: "Auto Shutdown in".to_string(),
            smwindow_position: "top-right".to_string(),
            background_img: "none".to_string(),
            countdown_timer: 100,
        };
        app_config.set_ui_config("device1", ui_config.clone()).unwrap();

        let retrieved_ui_config = app_config.get_ui_config("device1").unwrap();
        assert_eq!(retrieved_ui_config.cafe_name, ui_config.cafe_name);
    }


}
