use rbatis::rbdc::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Debug, bon::Builder, Serialize, Deserialize)]
pub struct ResetPassword {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub consumed: bool,
    pub consumed_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
