use std::sync::Arc;

use domain::repositories::phone_number_repository::PhoneNumberRepository;
use domain::{models::phone_number::PhoneNumber, repositories::RepositoryError};

use rbatis::{RBatis, async_trait};
use rbs::value;

use crate::database;
use crate::database::models::UuidExt;
use crate::repositories::RbsErrorExt;

#[derive(Debug, bon::Builder)]
pub struct PhoneNumberRepositoryImpl {
    pool: Arc<RBatis>,
}

#[async_trait]
impl PhoneNumberRepository for PhoneNumberRepositoryImpl {
    async fn create_phone_number(&self, phone_number: &PhoneNumber) -> Result<(), RepositoryError> {
        let new_phone_number = database::models::phone_number::PhoneNumber::from(phone_number);

        database::models::phone_number::PhoneNumber::insert(self.pool.as_ref(), &new_phone_number)
            .await
            .map_err(|e| e.to_repository_error())?;

        Ok(())
    }

    async fn find_by_id(
        &self,
        user_id: &uuid::Uuid,
        id: &uuid::Uuid,
    ) -> Result<PhoneNumber, RepositoryError> {
        let user_id_db = user_id.into_db();
        let id_db = id.into_db();
        let phone_number = database::models::phone_number::PhoneNumber::select_by_map(
            self.pool.as_ref(),
            value! { "id": id_db, "user_id": user_id_db },
        )
        .await
        .map_err(|e| e.to_repository_error())?
        .into_iter()
        .next()
        .ok_or(RepositoryError::NotFound)?;

        Ok(PhoneNumber::from(&phone_number))
    }

    async fn list_by_user_id(
        &self,
        user_id: &uuid::Uuid,
    ) -> Result<Vec<PhoneNumber>, RepositoryError> {
        let user_id_db = user_id.into_db();
        let records = database::models::phone_number::PhoneNumber::select_by_map(
            self.pool.as_ref(),
            value! { "user_id": user_id_db },
        )
        .await
        .map_err(|e| e.to_repository_error())?;

        let mut phone_numbers = records.iter().map(PhoneNumber::from).collect::<Vec<_>>();
        phone_numbers.sort_by(|a, b| a.name.cmp(&b.name).then_with(|| a.id.cmp(&b.id)));

        Ok(phone_numbers)
    }

    async fn delete_phone_number(
        &self,
        user_id: &uuid::Uuid,
        id: &uuid::Uuid,
    ) -> Result<(), RepositoryError> {
        let user_id_db = user_id.into_db();
        let id_db = id.into_db();
        self.find_by_id(user_id, id).await?;

        database::models::phone_number::PhoneNumber::delete_by_map(
            self.pool.as_ref(),
            value! { "id": id_db, "user_id": user_id_db },
        )
        .await
        .map_err(|e| e.to_repository_error())?;

        Ok(())
    }
}
