use super::entity::*;
use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use uuid::Uuid;

#[async_trait]
pub trait TeamRepository: Send + Sync {
	async fn create(
		&self,
		id: Uuid,
		leader_id: Uuid,
		input: CreateTeamInput,
	) -> Result<TeamEntity, AppError>;
	async fn find_by_id(&self, id: Uuid) -> Result<Option<TeamEntity>, AppError>;
	async fn browse(
		&self,
		input: BrowseTeamsInput,
	) -> Result<(Vec<TeamEntity>, i64), AppError>;
	async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<TeamEntity>, AppError>;
	async fn update(
		&self,
		id: Uuid,
		input: UpdateTeamInput,
	) -> Result<TeamEntity, AppError>;
	async fn delete(&self, id: Uuid) -> Result<bool, AppError>;
	async fn get_members(
		&self,
		team_id: Uuid,
	) -> Result<Vec<TeamMemberEntity>, AppError>;
	async fn get_leader(
		&self,
		leader_id: Uuid,
	) -> Result<Option<TeamUserInfo>, AppError>;
	async fn add_member(
		&self,
		team_id: Uuid,
		user_id: Uuid,
		role: &str,
	) -> Result<(), AppError>;
	async fn remove_member(
		&self,
		team_id: Uuid,
		user_id: Uuid,
	) -> Result<(), AppError>;
	async fn get_member_count(&self, team_id: Uuid) -> Result<i64, AppError>;
	async fn is_member(&self, team_id: Uuid, user_id: Uuid) -> Result<bool, AppError>;
	async fn is_leader(&self, team_id: Uuid, user_id: Uuid) -> Result<bool, AppError>;
	async fn user_active_team_name(
		&self,
		user_id: Uuid,
	) -> Result<Option<String>, AppError>;
	async fn team_has_submission(&self, team_id: Uuid) -> Result<bool, AppError>;
	async fn reject_pending_invitations_for_user(
		&self,
		user_id: Uuid,
	) -> Result<(), AppError>;
	async fn reject_pending_join_requests_for_user(
		&self,
		user_id: Uuid,
	) -> Result<(), AppError>;
	async fn get_leaders_batch(
		&self,
		leader_ids: Vec<Uuid>,
	) -> Result<Vec<TeamUserInfo>, AppError>;
	async fn get_member_counts_batch(
		&self,
		team_ids: Vec<Uuid>,
	) -> Result<Vec<(Uuid, i64)>, AppError>;
	async fn get_submitted_team_ids(
		&self,
		team_ids: Vec<Uuid>,
	) -> Result<Vec<Uuid>, AppError>;
}
