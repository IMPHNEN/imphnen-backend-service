use super::postgres_session_queries;
use crate::sessions::domain::{
	repository::SessionRepository, session::SessionEntity,
};
use async_trait::async_trait;
use imphnen_entities::seaorm::auth::sessions::{
	ActiveModel as SessionActiveModel, Entity as SessionsEntity, Model as SessionModel,
};
use imphnen_utils::AppError;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, IntoActiveModel};
use std::sync::Arc;
use uuid::Uuid;

pub fn model_to_entity(model: SessionModel) -> SessionEntity {
	SessionEntity {
		id: model.id,
		mentor_id: model.mentor_id,
		mentee_id: model.mentee_id,
		topic: model.topic,
		description: model.description,
		scheduled_at: model.scheduled_at,
		duration_minutes: model.duration_minutes,
		meeting_link: model.meeting_link,
		session_type: model.session_type,
		status: model.status,
		feedback: model.feedback,
		rating: model.rating,
		feedback_submitted_at: model.feedback_submitted_at,
		created_at: model.created_at,
		updated_at: model.updated_at,
	}
}

pub struct PostgresSessionRepository {
	pub db: Arc<DatabaseConnection>,
}

impl PostgresSessionRepository {
	pub fn new(db: DatabaseConnection) -> Self {
		Self { db: Arc::new(db) }
	}
}

#[async_trait]
impl SessionRepository for PostgresSessionRepository {
	async fn create(&self, entity: SessionEntity) -> Result<SessionEntity, AppError> {
		let active_model = SessionActiveModel {
			id: ActiveValue::Set(entity.id),
			mentor_id: ActiveValue::Set(entity.mentor_id),
			mentee_id: ActiveValue::Set(entity.mentee_id),
			topic: ActiveValue::Set(entity.topic.clone()),
			description: ActiveValue::Set(entity.description.clone()),
			scheduled_at: ActiveValue::Set(entity.scheduled_at),
			duration_minutes: ActiveValue::Set(entity.duration_minutes),
			meeting_link: ActiveValue::Set(entity.meeting_link.clone()),
			session_type: ActiveValue::Set(entity.session_type.clone()),
			status: ActiveValue::Set(entity.status.clone()),
			feedback: ActiveValue::Set(entity.feedback.clone()),
			rating: ActiveValue::Set(entity.rating),
			feedback_submitted_at: ActiveValue::Set(entity.feedback_submitted_at),
			created_at: ActiveValue::Set(entity.created_at),
			updated_at: ActiveValue::Set(entity.updated_at),
		};

		let model: SessionModel = active_model
			.insert(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		Ok(model_to_entity(model))
	}

	async fn find_by_id(&self, id: Uuid) -> Result<Option<SessionEntity>, AppError> {
		let model = SessionsEntity::find_by_id(id)
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		Ok(model.map(model_to_entity))
	}

	async fn find_by_mentor_id(
		&self,
		mentor_id: Uuid,
		status_filter: Option<String>,
	) -> Result<Vec<SessionEntity>, AppError> {
		postgres_session_queries::find_by_mentor_id(&self.db, mentor_id, status_filter)
			.await
	}

	async fn find_by_mentee_id(
		&self,
		mentee_id: Uuid,
		status_filter: Option<String>,
	) -> Result<Vec<SessionEntity>, AppError> {
		postgres_session_queries::find_by_mentee_id(&self.db, mentee_id, status_filter)
			.await
	}

	async fn find_booked_dates(
		&self,
		mentor_id: Uuid,
	) -> Result<Vec<String>, AppError> {
		postgres_session_queries::find_booked_dates(&self.db, mentor_id).await
	}

	async fn update(
		&self,
		id: Uuid,
		entity: SessionEntity,
	) -> Result<SessionEntity, AppError> {
		let model = SessionsEntity::find_by_id(id)
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?
			.ok_or_else(|| AppError::NotFoundError("Session not found".to_string()))?;

		let mut active_model = model.into_active_model();

		active_model.topic = ActiveValue::Set(entity.topic);
		active_model.description = ActiveValue::Set(entity.description);
		active_model.scheduled_at = ActiveValue::Set(entity.scheduled_at);
		active_model.duration_minutes = ActiveValue::Set(entity.duration_minutes);
		active_model.meeting_link = ActiveValue::Set(entity.meeting_link);
		active_model.session_type = ActiveValue::Set(entity.session_type);
		active_model.status = ActiveValue::Set(entity.status);
		active_model.feedback = ActiveValue::Set(entity.feedback);
		active_model.rating = ActiveValue::Set(entity.rating);
		active_model.feedback_submitted_at =
			ActiveValue::Set(entity.feedback_submitted_at);
		active_model.updated_at = ActiveValue::Set(entity.updated_at);

		let updated: SessionModel = active_model
			.update(self.db.as_ref())
			.await
			.map_err(|e: sea_orm::DbErr| AppError::InternalServerError(e.to_string()))?;

		Ok(model_to_entity(updated))
	}

	async fn delete(&self, id: Uuid) -> Result<(), AppError> {
		let model = SessionsEntity::find_by_id(id)
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?
			.ok_or_else(|| AppError::NotFoundError("Session not found".to_string()))?;

		model
			.into_active_model()
			.delete(self.db.as_ref())
			.await
			.map_err(|e: sea_orm::DbErr| AppError::InternalServerError(e.to_string()))?;

		Ok(())
	}

	async fn count_by_mentor(
		&self,
		mentor_id: Uuid,
		status_filter: Option<String>,
	) -> Result<usize, AppError> {
		postgres_session_queries::count_by_mentor(&self.db, mentor_id, status_filter)
			.await
	}

	async fn count_by_mentee(
		&self,
		mentee_id: Uuid,
		status_filter: Option<String>,
	) -> Result<usize, AppError> {
		postgres_session_queries::count_by_mentee(&self.db, mentee_id, status_filter)
			.await
	}

	async fn find_all_paginated(
		&self,
		params: PaginationParams,
	) -> Result<PaginatorResponse<SessionEntity>, AppError> {
		postgres_session_queries::find_all_paginated(&self.db, params).await
	}
}
