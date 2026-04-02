use crate::users::domain::entity::{HackathonUserEntity, UpdateUserInput};
use crate::users::domain::repository::HackathonUserRepository;
use crate::users::domain::service::HackathonUserService;
use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use std::sync::Arc;
use uuid::Uuid;

pub struct HackathonUserServiceImpl {
	repo: Arc<dyn HackathonUserRepository>,
}

impl HackathonUserServiceImpl {
	pub fn new(repo: Arc<dyn HackathonUserRepository>) -> Self {
		Self { repo }
	}
}

#[async_trait]
impl HackathonUserService for HackathonUserServiceImpl {
	async fn get_user(&self, id: Uuid) -> Result<HackathonUserEntity, AppError> {
		self.repo.find_by_id(id).await
	}

	async fn update_user(
		&self,
		id: Uuid,
		input: UpdateUserInput,
	) -> Result<HackathonUserEntity, AppError> {
		self.repo.update(id, input).await
	}

	async fn get_user_teams(
		&self,
		user_id: Uuid,
	) -> Result<Vec<serde_json::Value>, AppError> {
		self.repo.get_user_teams(user_id).await
	}
}
