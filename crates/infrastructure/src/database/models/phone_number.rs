use rbatis::rbdc::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Debug, bon::Builder, Serialize, Deserialize)]
pub struct PhoneNumber {
    pub id: Uuid,
    pub name: String,
    pub phone: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
