use async_trait::async_trait;

use crate::{models::user::User, repositories::RepositoryError};

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create_user(&self, user: &User) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: &uuid::Uuid) -> Result<User, RepositoryError>;
    async fn find_by_email(&self, email: &str) -> Result<User, RepositoryError>;
    async fn list(&self, page: u64, limit: u64) -> Result<Vec<User>, RepositoryError>;
    async fn update_user(&self, new_data: &User) -> Result<(), RepositoryError>;
    async fn delete_user(&self, id: &uuid::Uuid) -> Result<(), RepositoryError>;
}
