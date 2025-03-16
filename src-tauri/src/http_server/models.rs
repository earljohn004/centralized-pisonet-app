use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Clone)]
pub struct RegisterRequest {
    pub pair_id: String,
    pub address: String,
    pub hwid: String,
}

#[derive(Serialize, Clone)]
pub struct RegisterResponse {
    pub status: bool,
    pub server_hwid: String,
    pub server_address: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddTimeRequest {
    pub credits: u8,
}

#[derive(Serialize, Clone)]
pub struct AddTimeResponse {
    pub status: bool,
    pub text: String,
}
