use serde::{Deserialize, Serialize};

/// Flash message props for Inertia.js temporary messages
/// Used for success/error notifications that persist across a single redirect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashProps {
    #[serde(rename = "type")]
    pub type_: String,
    pub message: String,
}

impl FlashProps {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            type_: "success".into(),
            message: message.into(),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            type_: "error".into(),
            message: message.into(),
        }
    }
}
