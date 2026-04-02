use async_trait::async_trait;
use uuid::Uuid;
use imphnen_utils::errors::AppError;
use super::entity::*;

#[async_trait]
pub trait InvitationRepository: Send + Sync {
    async fn create(
        &self,
        invitation_id: Uuid,
        team_id: Uuid,
        inviter_id: Uuid,
        invitee_email: &str,
    ) -> Result<InvitationEntity, AppError>;

    async fn find_by_id(&self, id: Uuid) -> Result<Option<InvitationEntity>, AppError>;

    async fn find_pending_by_email(&self, email: &str) -> Result<Vec<InvitationWithDetails>, AppError>;

    async fn update_status(&self, id: Uuid, status: &str) -> Result<(), AppError>;

    async fn reject_pending_for_email_except(&self, email: &str, except_id: Uuid) -> Result<(), AppError>;

    async fn add_team_member(&self, team_id: Uuid, user_id: Uuid) -> Result<(), AppError>;

    async fn reject_pending_join_requests_for_user(&self, user_id: Uuid) -> Result<(), AppError>;

    async fn get_team_leader_id(&self, team_id: Uuid) -> Result<Option<Uuid>, AppError>;

    async fn get_team_name(&self, team_id: Uuid) -> Result<Option<String>, AppError>;

    async fn get_user_email(&self, user_id: Uuid) -> Result<Option<String>, AppError>;

    async fn get_inviter_name(&self, user_id: Uuid) -> Result<Option<String>, AppError>;

    async fn active_member_count(&self, team_id: Uuid) -> Result<i64, AppError>;

    async fn team_has_submission(&self, team_id: Uuid) -> Result<bool, AppError>;

    async fn user_active_team_name(&self, user_id: Uuid) -> Result<Option<String>, AppError>;
}
