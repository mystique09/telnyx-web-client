use std::net::Ipv4Addr;

use crate::config::ConfigError;

#[derive(Debug, bon::Builder)]
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

        Ok(Self::builder().host(host).port(port).build())
    }

    pub fn addrs(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
