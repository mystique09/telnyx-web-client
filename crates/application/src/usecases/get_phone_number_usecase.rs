use std::sync::Arc;

use crate::usecases::UsecaseError;
use domain::{
    models::phone_number::PhoneNumber, repositories::phone_number_repository::PhoneNumberRepository,
};

#[derive(bon::Builder)]
pub struct GetPhoneNumberUsecase {
    phone_number_repository: Arc<dyn PhoneNumberRepository>,
}

impl GetPhoneNumberUsecase {
    pub async fn execute(
        &self,
        user_id: uuid::Uuid,
        phone_number_id: uuid::Uuid,
    ) -> Result<PhoneNumber, UsecaseError> {
        let phone_number = self
            .phone_number_repository
            .find_by_id(&user_id, &phone_number_id)
            .await?;

        Ok(phone_number)
    }
}
