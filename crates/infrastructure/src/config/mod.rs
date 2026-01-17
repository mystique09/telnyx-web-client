pub mod database;
pub mod web;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Env var {0} not found")]
    EnvVarNotFound(String),
    #[error("Env var {0} not valid")]
    EnvVarNotValid(String),
}
