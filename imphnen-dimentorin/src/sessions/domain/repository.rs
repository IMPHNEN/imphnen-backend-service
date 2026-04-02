use super::session::SessionEntity;
use async_trait::async_trait;
use imphnen_utils::AppError;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use uuid::Uuid;

#[async_trait]
pub trait SessionRepository: Send + Sync {
	async fn create(&self, entity: SessionEntity) -> Result<SessionEntity, AppError>;

	async fn find_by_id(&self, id: Uuid) -> Result<Option<SessionEntity>, AppError>;

	async fn find_by_mentor_id(
		&self,
		mentor_id: Uuid,
		status_filter: Option<String>,
	) -> Result<Vec<SessionEntity>, AppError>;

	async fn find_by_mentee_id(
		&self,
		mentee_id: Uuid,
		status_filter: Option<String>,
	) -> Result<Vec<SessionEntity>, AppError>;

	async fn find_booked_dates(
		&self,
		mentor_id: Uuid,
	) -> Result<Vec<String>, AppError>;

	async fn update(
		&self,
		id: Uuid,
		entity: SessionEntity,
	) -> Result<SessionEntity, AppError>;

	async fn delete(&self, id: Uuid) -> Result<(), AppError>;

	async fn count_by_mentor(
		&self,
		mentor_id: Uuid,
		status_filter: Option<String>,
	) -> Result<usize, AppError>;

	async fn count_by_mentee(
		&self,
		mentee_id: Uuid,
		status_filter: Option<String>,
	) -> Result<usize, AppError>;

	async fn find_all_paginated(
		&self,
		params: PaginationParams,
	) -> Result<PaginatorResponse<SessionEntity>, AppError>;
}
