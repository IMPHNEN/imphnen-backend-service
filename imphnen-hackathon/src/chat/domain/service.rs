use super::entity::*;
use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use uuid::Uuid;

#[async_trait]
pub trait ChatService: Send + Sync {
	async fn get_team_messages(
		&self,
		team_id: Uuid,
		user_id: Uuid,
	) -> Result<Vec<MessageWithUser>, AppError>;

	async fn send_message(
		&self,
		team_id: Uuid,
		user_id: Uuid,
		input: SendMessageInput,
	) -> Result<MessageWithUser, AppError>;

	async fn delete_message(
		&self,
		message_id: Uuid,
		user_id: Uuid,
	) -> Result<(), AppError>;
}
