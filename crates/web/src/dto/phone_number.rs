use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreatePhoneNumberRequest {
    pub name: String,
    pub phone: String,
}

#[derive(Debug, Serialize)]
pub struct CreatePhoneNumberResponse {
    pub id: uuid::Uuid,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PhoneNumberProps {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub name: String,
    pub phone: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<&domain::models::phone_number::PhoneNumber> for PhoneNumberProps {
    fn from(value: &domain::models::phone_number::PhoneNumber) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            name: value.name.to_owned(),
            phone: value.phone.to_owned(),
            created_at: value.created_at.to_string(),
            updated_at: value.updated_at.to_string(),
        }
    }
}
