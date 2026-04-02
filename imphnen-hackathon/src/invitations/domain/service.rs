use async_trait::async_trait;
use uuid::Uuid;
use imphnen_utils::errors::AppError;
use super::entity::*;

#[async_trait]
pub trait InvitationService: Send + Sync {
    async fn invite_member(
        &self,
        team_id: Uuid,
        inviter_id: Uuid,
        input: CreateInvitationInput,
    ) -> Result<InvitationWithDetails, AppError>;

    async fn invite_member_for_team(
        &self,
        team_id: Uuid,
        inviter_id: Uuid,
        input: CreateInvitationInput,
    ) -> Result<InvitationWithDetails, AppError>;

    async fn get_my_invitations(&self, user_id: Uuid) -> Result<Vec<InvitationWithDetails>, AppError>;

    async fn respond_to_invitation(
        &self,
        invitation_id: Uuid,
        user_id: Uuid,
        accept: bool,
    ) -> Result<(), AppError>;
}
