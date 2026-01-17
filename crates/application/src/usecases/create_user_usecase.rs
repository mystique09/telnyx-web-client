use garde::Validate;
use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{commands::SignupCommand, responses::SignupResult, usecases::UsecaseError};
use domain::{
    repositories::user_repository::UserRepository, traits::password_hasher::PasswordHasher,
};

#[derive(bon::Builder)]
pub struct CreateUserUsecase {
    user_repository: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
}

impl CreateUserUsecase {
    pub async fn execute(&self, cmd: SignupCommand) -> Result<SignupResult, UsecaseError> {
        cmd.validate()?;
        cmd.validate_passwords_match()?;

        let hashed_password = self.password_hasher.hash(&cmd.password).await?;

        let user_id = Uuid::new_v4();
        let now = OffsetDateTime::now_utc();

        let user = domain::models::user::User::builder()
            .id(user_id)
            .email(cmd.email)
            .hash(hashed_password.hash)
            .salt(hashed_password.salt)
            .email_verified(false)
            .maybe_email_verified_at(None)
            .created_at(now)
            .updated_at(now)
            .build();

        self.user_repository.create_user(&user).await?;

        Ok(SignupResult {
            id: user_id,
            email: user.email,
            created_at: now,
        })
    }
}
