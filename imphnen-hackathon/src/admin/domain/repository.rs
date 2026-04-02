use super::entity::*;
use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use uuid::Uuid;

#[async_trait]
pub trait AdminRepository: Send + Sync {
	async fn list_users(
		&self,
		page: i64,
		limit: i64,
		search: Option<String>,
	) -> Result<(Vec<AdminUserRow>, i64), AppError>;
	async fn get_user(&self, user_id: Uuid) -> Result<Option<AdminUserRow>, AppError>;
	async fn set_admin(&self, user_id: Uuid, is_admin: bool) -> Result<(), AppError>;
	async fn delete_user(&self, user_id: Uuid) -> Result<(), AppError>;
	async fn list_teams(
		&self,
		page: i64,
		limit: i64,
		search: Option<String>,
	) -> Result<(Vec<AdminTeamRow>, i64), AppError>;
	async fn delete_team(&self, team_id: Uuid) -> Result<(), AppError>;
	async fn list_submissions(
		&self,
		page: i64,
		limit: i64,
		status: Option<String>,
	) -> Result<(Vec<AdminSubmissionRow>, i64), AppError>;
	async fn set_winner(
		&self,
		team_id: Uuid,
		rank: i32,
		prize: Option<String>,
	) -> Result<(), AppError>;
	async fn remove_winner(&self, team_id: Uuid) -> Result<(), AppError>;
	async fn list_winners(&self) -> Result<Vec<WinnerRow>, AppError>;
}
