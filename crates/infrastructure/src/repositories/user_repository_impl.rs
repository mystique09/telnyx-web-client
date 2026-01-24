use std::sync::Arc;

use domain::repositories::user_repository::UserRepository;
use domain::{models::user::User, repositories::RepositoryError};

use rbatis::rbdc::DateTime;
use rbatis::{PageRequest, RBatis, async_trait};
use rbs::value;

use crate::database;
use crate::repositories::RbsErrorExt;

#[derive(Debug, bon::Builder)]
pub struct UserRepositoryImpl {
    pool: Arc<RBatis>,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create_user(&self, user: &User) -> Result<(), RepositoryError> {
        let new_user_db = database::models::user::User::from(user);

        database::models::user::User::insert(self.pool.as_ref(), &new_user_db)
            .await
            .map_err(|e| e.to_repository_error())?;

        todo!()
    }

    async fn find_by_id(&self, id: &uuid::Uuid) -> Result<User, RepositoryError> {
        let user =
            database::models::user::User::select_by_map(self.pool.as_ref(), value! {"id": id})
                .await
                .map_err(|e| e.to_repository_error())?;
        let user = user.first().ok_or(RepositoryError::NotFound)?;

        Ok(User::from(user))
    }

    async fn find_by_email(&self, email: &str) -> Result<User, RepositoryError> {
        let users = database::models::user::User::select_by_map(
            self.pool.as_ref(),
            value! {"email": email},
        )
        .await
        .map_err(|e| e.to_repository_error())?;

        let user = users.first().ok_or(RepositoryError::NotFound)?;
        Ok(User::from(user))
    }

    async fn list(&self, page: u64, limit: u64) -> Result<Vec<User>, RepositoryError> {
        let page = database::models::user::User::list_users(
            self.pool.as_ref(),
            &PageRequest::new(page, limit),
        )
        .await
        .map_err(|e| e.to_repository_error())?;

        let users = page
            .records
            .iter()
            .map(|u| User::from(u))
            .collect::<Vec<_>>();

        Ok(users)
    }

    async fn update_user(&self, new_data: &User) -> Result<(), RepositoryError> {
        let mut existing = database::models::user::User::select_by_map(
            self.pool.as_ref(),
            value! {"id": new_data.id},
        )
        .await
        .map_err(|e| e.to_repository_error())?
        .into_iter()
        .next()
        .ok_or(RepositoryError::NotFound)?;

        // TODO: find a better way to do email update
        if existing.email != new_data.email {
            existing.email = new_data.email.to_owned();
            existing.email_verified = false;
            existing.email_verified_at = None;
        }

        if existing.hash != new_data.hash {
            existing.hash = new_data.hash.to_owned();
        }

        existing.updated_at = DateTime::now();

        database::models::user::User::update_by_map(
            self.pool.as_ref(),
            &existing,
            value! { "id": new_data.id },
        )
        .await
        .map_err(|e| e.to_repository_error())?;

        Ok(())
    }

    async fn delete_user(&self, id: &uuid::Uuid) -> Result<(), RepositoryError> {
        database::models::user::User::delete_by_map(self.pool.as_ref(), value! { "id": id })
            .await
            .map_err(|e| e.to_repository_error())?;

        Ok(())
    }
}
