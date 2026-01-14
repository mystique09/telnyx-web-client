use std::net::Ipv4Addr;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Env var {0} not found")]
    EnvVarNotFound(String),
    #[error("Env var {0} not valid")]
    EnvVarNotValid(String),
}

pub struct WebConfig {
    pub host: Ipv4Addr,
    pub port: u16,
}

impl WebConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let host = std::env::var("HOST")
            .map_err(|_| ConfigError::EnvVarNotFound("HOST".to_string()))?
            .parse::<Ipv4Addr>()
            .map_err(|_| ConfigError::EnvVarNotValid("HOST".to_string()))?;

        let port = std::env::var("PORT")
            .map_err(|_| ConfigError::EnvVarNotFound("PORT".to_string()))?
            .parse::<u16>()
            .map_err(|_| ConfigError::EnvVarNotValid("PORT".to_string()))?;

        Ok(Self { host, port })
    }

    pub fn addrs(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
