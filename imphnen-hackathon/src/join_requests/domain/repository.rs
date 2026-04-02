use super::entity::*;
use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use uuid::Uuid;

#[async_trait]
pub trait JoinRequestRepository: Send + Sync {
	async fn create(
		&self,
		id: Uuid,
		team_id: Uuid,
		user_id: Uuid,
		message: &str,
	) -> Result<JoinRequestEntity, AppError>;

	async fn find_by_id(
		&self,
		id: Uuid,
	) -> Result<Option<JoinRequestEntity>, AppError>;

	async fn find_by_user(
		&self,
		user_id: Uuid,
	) -> Result<Vec<JoinRequestWithDetails>, AppError>;

	async fn find_pending_by_team(
		&self,
		team_id: Uuid,
	) -> Result<Vec<JoinRequestWithDetails>, AppError>;

	async fn update_status(&self, id: Uuid, status: &str) -> Result<(), AppError>;

	async fn add_team_member(
		&self,
		team_id: Uuid,
		user_id: Uuid,
	) -> Result<(), AppError>;

	async fn reject_pending_invitations_for_user(
		&self,
		user_id: Uuid,
	) -> Result<(), AppError>;

	async fn reject_other_pending_for_user(
		&self,
		user_id: Uuid,
		except_id: Uuid,
	) -> Result<(), AppError>;

	async fn get_team_leader_id(
		&self,
		team_id: Uuid,
	) -> Result<Option<Uuid>, AppError>;

	async fn get_user_email(&self, user_id: Uuid) -> Result<Option<String>, AppError>;

	async fn team_exists(&self, team_id: Uuid) -> Result<bool, AppError>;

	async fn team_has_submission(&self, team_id: Uuid) -> Result<bool, AppError>;

	async fn user_active_team_name(
		&self,
		user_id: Uuid,
	) -> Result<Option<String>, AppError>;

	async fn active_member_count(&self, team_id: Uuid) -> Result<i64, AppError>;

	async fn pending_request_exists(
		&self,
		team_id: Uuid,
		user_id: Uuid,
	) -> Result<bool, AppError>;
}
