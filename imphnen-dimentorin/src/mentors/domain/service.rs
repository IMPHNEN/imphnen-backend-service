use super::mentor::MentorEntity;
use super::mentor_types::{
	MentorDetail, MentorListPage, MentorRegisterCommand, MentorRegistered,
	MentorUpdateCommand, MentorVerifyCommand,
};
use async_trait::async_trait;
use imphnen_utils::AppError;
use paginator_rs::PaginationParams;
use uuid::Uuid;

#[async_trait]
pub trait MentorService: Send + Sync {
	async fn list(&self, params: PaginationParams)
	-> Result<MentorListPage, AppError>;

	async fn get_by_id(&self, id: Uuid) -> Result<MentorDetail, AppError>;

	async fn get_by_email(&self, email: &str) -> Result<MentorDetail, AppError>;

	async fn register(
		&self,
		cmd: MentorRegisterCommand,
	) -> Result<MentorRegistered, AppError>;

	async fn update(
		&self,
		id: Uuid,
		cmd: MentorUpdateCommand,
	) -> Result<MentorDetail, AppError>;

	async fn update_me(
		&self,
		email: &str,
		cmd: MentorUpdateCommand,
	) -> Result<MentorDetail, AppError>;

	async fn delete(&self, id: Uuid) -> Result<(), AppError>;

	async fn verify(
		&self,
		id: Uuid,
		cmd: MentorVerifyCommand,
	) -> Result<MentorDetail, AppError>;

	async fn get_status(&self, email: &str) -> Result<String, AppError>;

	async fn get_entity_by_id(
		&self,
		id: Uuid,
		include_deleted: bool,
	) -> Result<MentorEntity, AppError>;
}
