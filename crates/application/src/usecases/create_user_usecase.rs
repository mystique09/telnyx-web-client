use std::sync::Arc;

use domain::{
    repositories::user_repository::UserRepository, traits::password_hasher::PasswordHasher,
};

use crate::usecases::UsecaseError;

#[derive(bon::Builder)]
pub struct CreateUserUsecase {
    user_repository: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
}

impl CreateUserUsecase {
    pub fn execute(&self) -> Result<(), UsecaseError> {
        Ok(())
    }
}
