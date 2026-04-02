use crate::permissions::domain::{
	PermissionEntity, PermissionRepository, PermissionService,
};
use async_trait::async_trait;
use imphnen_utils::AppError;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use std::sync::Arc;
use uuid::Uuid;

pub struct PermissionServiceImpl {
	repo: Arc<dyn PermissionRepository>,
}

impl PermissionServiceImpl {
	pub fn new(repo: Arc<dyn PermissionRepository>) -> Self {
		Self { repo }
	}
}

#[async_trait]
impl PermissionService for PermissionServiceImpl {
	async fn list(
		&self,
		params: PaginationParams,
	) -> Result<PaginatorResponse<PermissionEntity>, AppError> {
		self.repo.find_all(params).await
	}

	async fn get(&self, id: String) -> Result<PermissionEntity, AppError> {
		self.repo.find_by_id(id).await
	}

	async fn create(&self, name: String) -> Result<String, AppError> {
		match self.repo.find_by_name(name.clone()).await {
			Ok(_) => {
				return Err(AppError::ConflictError(
					"Permission name already exists".into(),
				));
			}
			Err(AppError::NotFoundError(_)) => {}
			Err(e) => return Err(e),
		}
		let entity = PermissionEntity {
			id: Uuid::new_v4(),
			name,
			is_deleted: false,
			created_at: None,
			updated_at: None,
		};
		self.repo.create(entity).await
	}

	async fn update(&self, entity: PermissionEntity) -> Result<String, AppError> {
		self.repo.update(entity).await
	}

	async fn delete(&self, id: String) -> Result<String, AppError> {
		self.repo.delete(id).await
	}
}
