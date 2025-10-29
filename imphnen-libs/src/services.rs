use async_trait::async_trait;
use imphnen_entities::UsersDetailQueryDto;
use crate::AppState;
use std::result::Result;
use surrealdb::sql::Thing;

#[async_trait]
pub trait UserLookupService: Send + Sync {
    async fn get_user_by_id_internal(
        &self,
        thing_id: &Thing,
        state: &AppState,
    ) -> Result<UsersDetailQueryDto, anyhow::Error>;
}

#[async_trait]
pub trait AuthRepositoryTrait: Send + Sync {
    async fn query_get_stored_user(
        &self,
        email: String,
    ) -> Result<UsersDetailQueryDto, anyhow::Error>;
}