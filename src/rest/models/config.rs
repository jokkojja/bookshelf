extern crate dotenv;

use dotenv::dotenv;
use std::net::SocketAddr;
use std::str::FromStr;

pub struct ApiConfig {
    pub address: SocketAddr,
}

impl ApiConfig {
    pub fn from_env() -> Self {
        dotenv().ok();
        let address: String = std::env::var("ADDRESS").expect("Address is not set");
        Self {
            address: SocketAddr::from_str(&address).expect("Invalid address"),
        }
    }
}
