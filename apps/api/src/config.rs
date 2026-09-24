use std::{env, net::SocketAddr};

pub const API_BIND_ADDR: &str = "API_BIND_ADDR";
pub const ENV_DATABASE_URL: &str = "DATABASE_URL";
pub const ENV_REDIS_URL: &str = "REDIS_URL";

pub struct Config {
    pub api_bind_addr: SocketAddr,
    pub redis_url: String,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Result<Self, String> {
        dotenvy::dotenv().ok();
        let api_bind_addr = required_env(API_BIND_ADDR)?
            .parse::<SocketAddr>()
            .map_err(|_| format!("Invalid value for {API_BIND_ADDR}: expected host:port"))?;

        let redis_url = required_env(ENV_REDIS_URL)?;
        let database_url = required_env(ENV_DATABASE_URL)?;

        Ok(Self {
            api_bind_addr,
            redis_url,
            database_url,
        })
    }

    pub fn addr(&self) -> String {
        self.api_bind_addr.to_string()
    }
}

fn required_env(name: &str) -> Result<String, String> {
    env::var(name).map_err(|_| format!("Missing required environment variable: {name}"))
}
