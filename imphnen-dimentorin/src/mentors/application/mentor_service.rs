use super::mentor_query_service::MentorQueryService;
use super::mentor_registration_service::MentorRegistrationService;
use super::mentor_update_service::MentorUpdateService;
use crate::mentors::domain::{
	MentorDetail, MentorEntity, MentorListPage, MentorRegisterCommand,
	MentorRegistered, MentorRepository, MentorService, MentorUpdateCommand,
	MentorVerifyCommand,
};
use async_trait::async_trait;
use imphnen_iam::roles::domain::RoleRepository;
use imphnen_iam::users::domain::UserRepository;
use imphnen_libs::AppState;
use imphnen_utils::AppError;
use paginator_rs::PaginationParams;
use std::sync::Arc;
use uuid::Uuid;

pub struct MentorServiceImpl {
	query: MentorQueryService,
	registration: MentorRegistrationService,
	update: MentorUpdateService,
}

impl MentorServiceImpl {
	pub fn new(
		repo: Arc<dyn MentorRepository>,
		state: Arc<AppState>,
		user_repo: Arc<dyn UserRepository>,
		role_repo: Arc<dyn RoleRepository>,
	) -> Self {
		Self {
			query: MentorQueryService {
				repo: Arc::clone(&repo),
				state: Arc::clone(&state),
			},
			registration: MentorRegistrationService {
				repo: Arc::clone(&repo),
				state: Arc::clone(&state),
				user_repo,
				role_repo,
			},
			update: MentorUpdateService {
				repo: Arc::clone(&repo),
				state: Arc::clone(&state),
			},
		}
	}
}

#[async_trait]
impl MentorService for MentorServiceImpl {
	async fn list(
		&self,
		params: PaginationParams,
	) -> Result<MentorListPage, AppError> {
		self.query.list(params).await
	}

	async fn get_by_id(&self, id: Uuid) -> Result<MentorDetail, AppError> {
		self.query.get_by_id(id).await
	}

	async fn get_by_email(&self, email: &str) -> Result<MentorDetail, AppError> {
		self.query.get_by_email(email).await
	}

	async fn register(
		&self,
		cmd: MentorRegisterCommand,
	) -> Result<MentorRegistered, AppError> {
		self.registration.register(cmd).await
	}

	async fn update(
		&self,
		id: Uuid,
		cmd: MentorUpdateCommand,
	) -> Result<MentorDetail, AppError> {
		self.update.update(id, cmd).await
	}

	async fn update_me(
		&self,
		email: &str,
		cmd: MentorUpdateCommand,
	) -> Result<MentorDetail, AppError> {
		self.update.update_me(email, cmd).await
	}

	async fn delete(&self, id: Uuid) -> Result<(), AppError> {
		self.registration.delete(id).await
	}

	async fn verify(
		&self,
		id: Uuid,
		cmd: MentorVerifyCommand,
	) -> Result<MentorDetail, AppError> {
		self.registration.verify(id, cmd).await
	}

	async fn get_status(&self, email: &str) -> Result<String, AppError> {
		self.query.get_status(email).await
	}

	async fn get_entity_by_id(
		&self,
		id: Uuid,
		include_deleted: bool,
	) -> Result<MentorEntity, AppError> {
		self.query.get_entity_by_id(id, include_deleted).await
	}
}
