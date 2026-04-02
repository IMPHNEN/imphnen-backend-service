use super::entity::{HackathonUserEntity, UpdateUserInput};
use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use uuid::Uuid;

#[async_trait]
pub trait HackathonUserRepository: Send + Sync {
	async fn find_by_id(&self, id: Uuid) -> Result<HackathonUserEntity, AppError>;
	async fn update(
		&self,
		id: Uuid,
		input: UpdateUserInput,
	) -> Result<HackathonUserEntity, AppError>;
	async fn get_user_teams(
		&self,
		user_id: Uuid,
	) -> Result<Vec<serde_json::Value>, AppError>;
}
