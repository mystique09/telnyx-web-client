use std::sync::Arc;

use crate::usecases::UsecaseError;
use domain::{
    models::phone_number::PhoneNumber, repositories::phone_number_repository::PhoneNumberRepository,
};

#[derive(bon::Builder)]
pub struct ListPhoneNumbersUsecase {
    phone_number_repository: Arc<dyn PhoneNumberRepository>,
}

impl ListPhoneNumbersUsecase {
    pub async fn execute(&self, user_id: uuid::Uuid) -> Result<Vec<PhoneNumber>, UsecaseError> {
        let phone_numbers = self
            .phone_number_repository
            .list_by_user_id(&user_id)
            .await?;

        Ok(phone_numbers)
    }
}
