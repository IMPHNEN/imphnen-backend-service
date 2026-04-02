use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, IntoActiveModel, Order, QueryOrder, PaginatorTrait};
use paginator_rs::{PaginationParams, SortDirection};
use paginator_utils::{PaginatorResponse, PaginatorResponseMeta};
use uuid::Uuid;
use imphnen_utils::AppError;
use imphnen_entities::seaorm::auth::sessions::{
    Entity as SessionsEntity,
    Column as SessionColumn,
    ActiveModel as SessionActiveModel,
    Model as SessionModel,
};
use crate::sessions::domain::{session::SessionEntity, repository::SessionRepository};

fn model_to_entity(model: SessionModel) -> SessionEntity {
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
    db: Arc<DatabaseConnection>,
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
        let mut query = SessionsEntity::find()
            .filter(SessionColumn::MentorId.eq(mentor_id))
            .order_by(SessionColumn::ScheduledAt, Order::Desc);

        if let Some(status) = status_filter {
            query = query.filter(SessionColumn::Status.eq(status));
        }

        let models = query
            .all(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(models.into_iter().map(model_to_entity).collect())
    }

    async fn find_by_mentee_id(
        &self,
        mentee_id: Uuid,
        status_filter: Option<String>,
    ) -> Result<Vec<SessionEntity>, AppError> {
        let mut query = SessionsEntity::find()
            .filter(SessionColumn::MenteeId.eq(mentee_id))
            .order_by(SessionColumn::ScheduledAt, Order::Desc);

        if let Some(status) = status_filter {
            query = query.filter(SessionColumn::Status.eq(status));
        }

        let models = query
            .all(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(models.into_iter().map(model_to_entity).collect())
    }

    async fn find_booked_dates(&self, mentor_id: Uuid) -> Result<Vec<String>, AppError> {
        let sessions = SessionsEntity::find()
            .filter(SessionColumn::MentorId.eq(mentor_id))
            .filter(SessionColumn::Status.is_in(["pending", "confirmed"]))
            .order_by(SessionColumn::ScheduledAt, Order::Asc)
            .all(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(sessions
            .into_iter()
            .map(|s| s.scheduled_at.to_rfc3339())
            .collect())
    }

    async fn update(&self, id: Uuid, entity: SessionEntity) -> Result<SessionEntity, AppError> {
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
        active_model.feedback_submitted_at = ActiveValue::Set(entity.feedback_submitted_at);
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
        let mut query = SessionsEntity::find()
            .filter(SessionColumn::MentorId.eq(mentor_id));

        if let Some(status) = status_filter {
            query = query.filter(SessionColumn::Status.eq(status));
        }

        let count = query
            .count(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(count as usize)
    }

    async fn count_by_mentee(
        &self,
        mentee_id: Uuid,
        status_filter: Option<String>,
    ) -> Result<usize, AppError> {
        let mut query = SessionsEntity::find()
            .filter(SessionColumn::MenteeId.eq(mentee_id));

        if let Some(status) = status_filter {
            query = query.filter(SessionColumn::Status.eq(status));
        }

        let count = query
            .count(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(count as usize)
    }

    async fn find_all_paginated(
        &self,
        params: PaginationParams,
    ) -> Result<PaginatorResponse<SessionEntity>, AppError> {
        let page = params.page.max(1);
        let per_page = params.per_page.clamp(1, 100);

        let query = match params.sort_direction {
            Some(SortDirection::Asc) => SessionsEntity::find()
                .order_by(SessionColumn::CreatedAt, Order::Asc),
            _ => SessionsEntity::find()
                .order_by(SessionColumn::CreatedAt, Order::Desc),
        };

        let paginator = query.paginate(self.db.as_ref(), per_page as u64);
        let total = paginator
            .num_items()
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        let sessions = paginator
            .fetch_page((page - 1) as u64)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let data = sessions.into_iter().map(model_to_entity).collect();
        let meta = PaginatorResponseMeta::new(page, per_page, total as u32);
        Ok(PaginatorResponse { data, meta })
    }
}
