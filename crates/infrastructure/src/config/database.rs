use crate::config::ConfigError;

#[derive(Debug, bon::Builder)]
pub struct DatabaseConfig {
    pub url: String,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let url = std::env::var("DATABASE_URL")
            .map_err(|_| ConfigError::EnvVarNotFound("missing DATABASE_URL".to_string()))?;

        Ok(Self::builder().url(url).build())
    }
}
