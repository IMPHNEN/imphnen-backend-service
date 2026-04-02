use crate::admin::domain::entity::*;
use crate::admin::domain::repository::AdminRepository;
use crate::admin::domain::service::AdminService;
use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use std::sync::Arc;
use uuid::Uuid;

pub struct AdminServiceImpl {
	repo: Arc<dyn AdminRepository>,
}

impl AdminServiceImpl {
	pub fn new(repo: Arc<dyn AdminRepository>) -> Self {
		Self { repo }
	}
}

#[async_trait]
impl AdminService for AdminServiceImpl {
	async fn list_users(
		&self,
		page: i64,
		limit: i64,
		search: Option<String>,
	) -> Result<(Vec<AdminUserRow>, i64), AppError> {
		self.repo.list_users(page, limit, search).await
	}

	async fn get_user(&self, user_id: Uuid) -> Result<AdminUserRow, AppError> {
		self
			.repo
			.get_user(user_id)
			.await?
			.ok_or_else(|| AppError::NotFoundError("User not found".to_string()))
	}

	async fn set_admin(&self, user_id: Uuid, is_admin: bool) -> Result<(), AppError> {
		self.repo.set_admin(user_id, is_admin).await
	}

	async fn delete_user(&self, user_id: Uuid) -> Result<(), AppError> {
		self.repo.delete_user(user_id).await
	}

	async fn list_teams(
		&self,
		page: i64,
		limit: i64,
		search: Option<String>,
	) -> Result<(Vec<AdminTeamRow>, i64), AppError> {
		self.repo.list_teams(page, limit, search).await
	}

	async fn delete_team(&self, team_id: Uuid) -> Result<(), AppError> {
		self.repo.delete_team(team_id).await
	}

	async fn list_submissions(
		&self,
		page: i64,
		limit: i64,
		status: Option<String>,
	) -> Result<(Vec<AdminSubmissionRow>, i64), AppError> {
		self.repo.list_submissions(page, limit, status).await
	}

	async fn set_winner(
		&self,
		team_id: Uuid,
		rank: i32,
		prize: Option<String>,
	) -> Result<(), AppError> {
		self.repo.set_winner(team_id, rank, prize).await
	}

	async fn remove_winner(&self, team_id: Uuid) -> Result<(), AppError> {
		self.repo.remove_winner(team_id).await
	}

	async fn list_winners(&self) -> Result<Vec<WinnerRow>, AppError> {
		self.repo.list_winners().await
	}
}
