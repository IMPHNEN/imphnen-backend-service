use async_trait::async_trait;
use uuid::Uuid;
use imphnen_utils::errors::AppError;
use super::entity::*;

#[async_trait]
pub trait SubmissionRepository: Send + Sync {
    async fn create(&self, team_id: Uuid, user_id: Uuid, input: CreateSubmissionInput) -> Result<SubmissionEntity, AppError>;
    async fn find_by_team(&self, team_id: Uuid) -> Result<Option<SubmissionEntity>, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<SubmissionEntity, AppError>;
    async fn update(&self, id: Uuid, input: UpdateSubmissionInput) -> Result<SubmissionEntity, AppError>;
    async fn update_status(&self, id: Uuid, status: &str) -> Result<SubmissionEntity, AppError>;
    async fn is_team_leader(&self, team_id: Uuid, user_id: Uuid) -> Result<bool, AppError>;
    async fn is_team_member(&self, team_id: Uuid, user_id: Uuid) -> Result<bool, AppError>;
    async fn team_member_count(&self, team_id: Uuid) -> Result<i64, AppError>;
}
