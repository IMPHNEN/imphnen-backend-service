use super::entity::*;
use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use uuid::Uuid;

#[async_trait]
pub trait ChatRepository: Send + Sync {
	async fn find_team_messages(
		&self,
		team_id: Uuid,
	) -> Result<Vec<MessageWithUser>, AppError>;

	async fn create_message(
		&self,
		id: Uuid,
		team_id: Uuid,
		user_id: Uuid,
		message: &str,
	) -> Result<MessageEntity, AppError>;

	async fn find_message_by_id(
		&self,
		id: Uuid,
	) -> Result<Option<MessageEntity>, AppError>;

	async fn delete_message(&self, id: Uuid) -> Result<bool, AppError>;

	async fn get_user_info(
		&self,
		user_id: Uuid,
	) -> Result<Option<(String, Option<String>)>, AppError>;

	async fn is_team_member(
		&self,
		team_id: Uuid,
		user_id: Uuid,
	) -> Result<bool, AppError>;

	async fn is_team_leader(
		&self,
		team_id: Uuid,
		user_id: Uuid,
	) -> Result<bool, AppError>;
}
