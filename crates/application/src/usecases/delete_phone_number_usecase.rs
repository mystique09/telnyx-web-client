use std::sync::Arc;

use crate::usecases::UsecaseError;
use domain::repositories::phone_number_repository::PhoneNumberRepository;

#[derive(bon::Builder)]
pub struct DeletePhoneNumberUsecase {
    phone_number_repository: Arc<dyn PhoneNumberRepository>,
}

impl DeletePhoneNumberUsecase {
    pub async fn execute(
        &self,
        user_id: uuid::Uuid,
        phone_number_id: uuid::Uuid,
    ) -> Result<(), UsecaseError> {
        self.phone_number_repository
            .delete_phone_number(&user_id, &phone_number_id)
            .await?;

        Ok(())
    }
}
