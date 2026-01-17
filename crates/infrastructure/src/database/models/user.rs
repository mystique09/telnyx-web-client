use rbatis::rbdc::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Debug, bon::Builder, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub hash: String,
    pub salt: String,
    pub email_verified: bool,
    pub email_verified_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
