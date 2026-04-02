use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use uuid::Uuid;

use super::entity::{UpdateUserInput, UserEntity};

#[async_trait]
pub trait UserRepository: Send + Sync {
	async fn find_by_id(&self, id: Uuid) -> Result<Option<UserEntity>, AppError>;
	async fn find_all(&self) -> Result<Vec<UserEntity>, AppError>;
	async fn update(
		&self,
		id: Uuid,
		input: UpdateUserInput,
	) -> Result<UserEntity, AppError>;
	async fn update_role(
		&self,
		id: Uuid,
		role: String,
	) -> Result<UserEntity, AppError>;
	async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
