use std::sync::Arc;
use uuid::Uuid;
use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use crate::chat::domain::entity::*;
use crate::chat::domain::repository::ChatRepository;
use crate::chat::domain::service::ChatService;

pub struct ChatServiceImpl {
    repo: Arc<dyn ChatRepository>,
}

impl ChatServiceImpl {
    pub fn new(repo: Arc<dyn ChatRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl ChatService for ChatServiceImpl {
    async fn get_team_messages(&self, team_id: Uuid, user_id: Uuid) -> Result<Vec<MessageWithUser>, AppError> {
        if !self.repo.is_team_member(team_id, user_id).await? {
            return Err(AppError::ForbiddenError("Only team members can view messages".to_string()));
        }
        self.repo.find_team_messages(team_id).await
    }

    async fn send_message(
        &self,
        team_id: Uuid,
        user_id: Uuid,
        input: SendMessageInput,
    ) -> Result<MessageWithUser, AppError> {
        if input.message.trim().is_empty() {
            return Err(AppError::BadRequestError("Message cannot be empty".to_string()));
        }
        if !self.repo.is_team_member(team_id, user_id).await? {
            return Err(AppError::ForbiddenError("Only team members can send messages".to_string()));
        }
        let user_info = self.repo.get_user_info(user_id).await?
            .ok_or_else(|| AppError::NotFoundError("User not found".to_string()))?;
        let id = Uuid::new_v4();
        let entity = self.repo.create_message(id, team_id, user_id, &input.message).await?;
        Ok(MessageWithUser {
            id: entity.id,
            team_id: entity.team_id,
            user_id: entity.user_id,
            user_fullname: user_info.0,
            user_avatar: user_info.1,
            message: entity.message,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        })
    }

    async fn delete_message(&self, message_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let message = self.repo.find_message_by_id(message_id).await?
            .ok_or_else(|| AppError::NotFoundError("Message not found".to_string()))?;
        let is_author = message.user_id == user_id;
        let is_leader = self.repo.is_team_leader(message.team_id, user_id).await?;
        if !is_author && !is_leader {
            return Err(AppError::ForbiddenError("You can only delete your own messages or messages as team leader".to_string()));
        }
        let deleted = self.repo.delete_message(message_id).await?;
        if !deleted {
            return Err(AppError::NotFoundError("Message not found".to_string()));
        }
        Ok(())
    }
}
