use async_trait::async_trait;
use uuid::Uuid;
use imphnen_utils::errors::AppError;
use super::entity::{HackathonUserEntity, UpdateUserInput};

#[async_trait]
pub trait HackathonUserService: Send + Sync {
    async fn get_user(&self, id: Uuid) -> Result<HackathonUserEntity, AppError>;
    async fn update_user(&self, id: Uuid, input: UpdateUserInput) -> Result<HackathonUserEntity, AppError>;
    async fn get_user_teams(&self, user_id: Uuid) -> Result<Vec<serde_json::Value>, AppError>;
}
