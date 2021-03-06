use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ConnectRequest {
    pub proto: String,
    pub addr: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AuthRequest {
    //pub key: String,
    pub method: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AuthResponse {
    pub success: bool,
    pub err: String,
    pub rand: u64,
    pub method: String,
}
