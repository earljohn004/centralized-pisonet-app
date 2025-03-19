use std::collections::HashMap;

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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Server {
    pub address: String,
    pub port: String,
    pub hwid: String,
    pub password: String,
    pub configpath: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct License {
    pub authorized: bool,
    #[serde(rename = "seat-serial")]
    pub seat_serial: String,
    pub identification: String,
    pub hwid: String,
}

impl AppConfig {
    pub fn default() -> Self {
        let mut devices = HashMap::new();

        let default_device_config = DeviceConfig {
            config: InnerConfig {
                client: HashMap::new(),
                server: Server {
                    address: "127.0.0.1".to_string(),
                    port: "8080".to_string(),
                    hwid: "default-hwid".to_string(),
                    password: "default-password".to_string(),
                    configpath: "C:\\ProgramData\\CPS\\config".to_string(),
                },
                license: License {
                    authorized: false,
                    seat_serial: "default-seat-serial".to_string(),
                    identification: "default-identification".to_string(),
                    hwid: "default-hwid".to_string(),
                },
            },
        };

        devices.insert("default-device".to_string(), default_device_config);

        AppConfig {
            version: "1.0".to_string(),
            devices,
        }
    }
}

