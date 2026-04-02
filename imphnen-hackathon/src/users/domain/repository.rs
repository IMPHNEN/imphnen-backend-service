use async_trait::async_trait;
use uuid::Uuid;
use imphnen_utils::errors::AppError;
use super::entity::{HackathonUserEntity, UpdateUserInput};

#[async_trait]
pub trait HackathonUserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<HackathonUserEntity, AppError>;
    async fn update(&self, id: Uuid, input: UpdateUserInput) -> Result<HackathonUserEntity, AppError>;
    async fn get_user_teams(&self, user_id: Uuid) -> Result<Vec<serde_json::Value>, AppError>;
}
