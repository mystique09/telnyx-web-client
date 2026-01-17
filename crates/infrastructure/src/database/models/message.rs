use rbatis::rbdc::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Debug, bon::Builder, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
