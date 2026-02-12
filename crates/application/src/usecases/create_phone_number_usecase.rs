use std::sync::Arc;

use time::OffsetDateTime;

use crate::{
    commands::CreatePhoneNumberCommand, responses::CreatePhoneNumberResult, usecases::UsecaseError,
};
use domain::{
    models::phone_number::PhoneNumber, repositories::phone_number_repository::PhoneNumberRepository,
};

#[derive(bon::Builder)]
pub struct CreatePhoneNumberUsecase {
    phone_number_repository: Arc<dyn PhoneNumberRepository>,
}

impl CreatePhoneNumberUsecase {
    pub async fn execute(
        &self,
        cmd: CreatePhoneNumberCommand,
    ) -> Result<CreatePhoneNumberResult, UsecaseError> {
        let now = OffsetDateTime::now_utc();
        let phone_number_id = uuid::Uuid::now_v7();
        let phone_number = PhoneNumber::builder()
            .id(phone_number_id)
            .user_id(cmd.user_id)
            .name(cmd.name)
            .phone(cmd.phone)
            .created_at(now)
            .updated_at(now)
            .build();

        self.phone_number_repository
            .create_phone_number(&phone_number)
            .await?;

        Ok(CreatePhoneNumberResult {
            id: phone_number_id,
        })
    }
}
