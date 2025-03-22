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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct License {
    pub authorized: bool,
    #[serde(rename = "seat-serial")]
    pub seat_serial: String,
    pub identification: String,
    pub hwid: String,
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
}
