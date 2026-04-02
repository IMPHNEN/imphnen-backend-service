use super::repository::UserListItem;
use super::user::UserEntity;
use async_trait::async_trait;
use imphnen_utils::AppError;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;

#[async_trait]
pub trait UserService: Send + Sync {
	async fn list(
		&self,
		params: PaginationParams,
	) -> Result<PaginatorResponse<UserListItem>, AppError>;
	async fn get(&self, id: String) -> Result<UserEntity, AppError>;
	async fn get_me(&self, user_id: String) -> Result<UserEntity, AppError>;
	async fn get_by_email(&self, email: String) -> Result<UserEntity, AppError>;
	async fn create(&self, entity: UserEntity) -> Result<UserEntity, AppError>;
	async fn update(&self, entity: UserEntity) -> Result<String, AppError>;
	async fn delete(&self, id: String) -> Result<String, AppError>;
	async fn set_active_status(
		&self,
		id: String,
		is_active: bool,
	) -> Result<String, AppError>;
	async fn update_password(
		&self,
		email: String,
		old_password: String,
		new_password: String,
	) -> Result<String, AppError>;
}
