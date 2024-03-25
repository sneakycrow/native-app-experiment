use std::net::SocketAddr;

use crate::db::get_dsn_from_env;

#[derive(Default)]
pub struct Config {
    pub db: DBConfig,
    pub server: ServerConfig,
}

pub struct DBConfig {
    pub url: String,
}

impl Default for DBConfig {
    fn default() -> Self {
        let url = get_dsn_from_env();
        DBConfig { url }
    }
}

pub struct ServerConfig {
    pub addr: SocketAddr,
}

impl Default for ServerConfig {
    fn default() -> Self {
        let addr: SocketAddr = "[::1]:50051".parse().unwrap();
        ServerConfig { addr }
    }
}
