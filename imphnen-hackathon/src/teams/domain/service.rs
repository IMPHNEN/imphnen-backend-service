use super::entity::*;
use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use uuid::Uuid;

#[async_trait]
pub trait TeamService: Send + Sync {
	async fn create_team(
		&self,
		user_id: Uuid,
		input: CreateTeamInput,
	) -> Result<TeamWithDetails, AppError>;
	async fn get_team_by_id(&self, team_id: Uuid)
	-> Result<TeamWithDetails, AppError>;
	async fn browse_teams(
		&self,
		input: BrowseTeamsInput,
	) -> Result<BrowseTeamsResult, AppError>;
	async fn get_user_teams(
		&self,
		user_id: Uuid,
	) -> Result<Vec<TeamWithDetails>, AppError>;
	async fn update_team(
		&self,
		team_id: Uuid,
		user_id: Uuid,
		input: UpdateTeamInput,
	) -> Result<TeamWithDetails, AppError>;
	async fn remove_team_member(
		&self,
		team_id: Uuid,
		user_id: Uuid,
		member_id: Uuid,
	) -> Result<(), AppError>;
	async fn leave_team(&self, team_id: Uuid, user_id: Uuid) -> Result<(), AppError>;
	async fn delete_team(&self, team_id: Uuid, user_id: Uuid) -> Result<(), AppError>;
}
