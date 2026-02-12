use async_trait::async_trait;

use crate::{models::phone_number::PhoneNumber, repositories::RepositoryError};

#[async_trait]
pub trait PhoneNumberRepository: Send + Sync + 'static {
    async fn create_phone_number(&self, phone_number: &PhoneNumber) -> Result<(), RepositoryError>;
    async fn find_by_id(
        &self,
        user_id: &uuid::Uuid,
        id: &uuid::Uuid,
    ) -> Result<PhoneNumber, RepositoryError>;
    async fn list_by_user_id(
        &self,
        user_id: &uuid::Uuid,
    ) -> Result<Vec<PhoneNumber>, RepositoryError>;
    async fn delete_phone_number(
        &self,
        user_id: &uuid::Uuid,
        id: &uuid::Uuid,
    ) -> Result<(), RepositoryError>;
}
