use async_trait::async_trait;
use uuid::Uuid;
use imphnen_utils::errors::AppError;
use super::entity::*;

#[async_trait]
pub trait JoinRequestService: Send + Sync {
    async fn create_join_request(
        &self,
        team_id: Uuid,
        user_id: Uuid,
        input: CreateJoinRequestInput,
    ) -> Result<JoinRequestWithDetails, AppError>;

    async fn get_my_join_requests(&self, user_id: Uuid) -> Result<Vec<JoinRequestWithDetails>, AppError>;

    async fn get_team_join_requests(
        &self,
        team_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<JoinRequestWithDetails>, AppError>;

    async fn respond_to_join_request(
        &self,
        request_id: Uuid,
        user_id: Uuid,
        accept: bool,
    ) -> Result<(), AppError>;
}
