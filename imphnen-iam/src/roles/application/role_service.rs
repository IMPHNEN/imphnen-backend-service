use crate::roles::domain::{RoleEntity, RoleRepository, RoleService};
use async_trait::async_trait;
use imphnen_utils::AppError;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use std::sync::Arc;
use uuid::Uuid;

pub struct RoleServiceImpl {
	repo: Arc<dyn RoleRepository>,
}

impl RoleServiceImpl {
	pub fn new(repo: Arc<dyn RoleRepository>) -> Self {
		Self { repo }
	}
}

#[async_trait]
impl RoleService for RoleServiceImpl {
	async fn list(
		&self,
		params: PaginationParams,
	) -> Result<PaginatorResponse<RoleEntity>, AppError> {
		self.repo.find_all(params).await
	}

	async fn get(&self, id: String) -> Result<RoleEntity, AppError> {
		self.repo.find_by_id(id).await
	}

	async fn create(
		&self,
		name: String,
		permissions: Vec<String>,
	) -> Result<RoleEntity, AppError> {
		match self.repo.find_by_name(name.clone()).await {
			Ok(_) => {
				return Err(AppError::ConflictError("Role name already exists".into()));
			}
			Err(AppError::NotFoundError(_)) => {}
			Err(e) => return Err(e),
		}
		let entity = RoleEntity {
			id: Uuid::new_v4(),
			name,
			permissions,
			..Default::default()
		};
		self.repo.create(entity).await
	}

	async fn update(
		&self,
		id: String,
		name: Option<String>,
		permissions: Option<Vec<String>>,
	) -> Result<String, AppError> {
		let existing = self.repo.find_by_id(id.clone()).await?;
		if let Some(ref new_name) = name {
			match self.repo.find_by_name(new_name.clone()).await {
				Ok(found) if found.id != existing.id => {
					return Err(AppError::ConflictError("Role name already exists".into()));
				}
				Ok(_) => {}
				Err(AppError::NotFoundError(_)) => {}
				Err(e) => return Err(e),
			}
		}

		self.repo.update(id, name, permissions).await
	}

	async fn delete(&self, id: String) -> Result<String, AppError> {
		self.repo.find_by_id(id.clone()).await?;
		self.repo.delete(id).await
	}
}
