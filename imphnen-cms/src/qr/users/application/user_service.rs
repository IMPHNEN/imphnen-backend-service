use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use std::sync::Arc;
use uuid::Uuid;

use crate::qr::users::domain::{
	entity::{UpdateUserInput, UserEntity},
	repository::UserRepository,
	service::QrUserService,
};

pub struct QrUserServiceImpl {
	repo: Arc<dyn UserRepository>,
}

impl QrUserServiceImpl {
	pub fn new(repo: Arc<dyn UserRepository>) -> Self {
		Self { repo }
	}
}

#[async_trait]
impl QrUserService for QrUserServiceImpl {
	async fn get_profile(&self, user_id: Uuid) -> Result<UserEntity, AppError> {
		self
			.repo
			.find_by_id(user_id)
			.await?
			.ok_or_else(|| AppError::NotFoundError("User not found".to_string()))
	}

	async fn update_profile(
		&self,
		user_id: Uuid,
		input: UpdateUserInput,
	) -> Result<UserEntity, AppError> {
		if let Some(ref email) = input.email
			&& email.trim().is_empty()
		{
			return Err(AppError::ValidationError(
				"Email cannot be empty".to_string(),
			));
		}
		self.repo.update(user_id, input).await
	}

	async fn list_all(&self) -> Result<Vec<UserEntity>, AppError> {
		self.repo.find_all().await
	}

	async fn update_role(
		&self,
		id: Uuid,
		role: String,
	) -> Result<UserEntity, AppError> {
		self.repo.update_role(id, role).await
	}

	async fn delete(&self, id: Uuid) -> Result<(), AppError> {
		self.repo.delete(id).await
	}
}
