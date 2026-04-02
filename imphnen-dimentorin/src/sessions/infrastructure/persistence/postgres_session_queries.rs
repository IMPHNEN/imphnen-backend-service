use super::postgres_session_repository::model_to_entity;
use crate::sessions::domain::session::SessionEntity;
use imphnen_entities::seaorm::auth::sessions::{
	Column as SessionColumn, Entity as SessionsEntity,
};
use imphnen_utils::AppError;
use paginator_rs::{PaginationParams, SortDirection};
use paginator_utils::{PaginatorResponse, PaginatorResponseMeta};
use sea_orm::prelude::*;
use sea_orm::{Order, PaginatorTrait, QueryOrder};
use std::sync::Arc;

pub async fn find_by_mentor_id(
	db: &Arc<DatabaseConnection>,
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
		.all(db.as_ref())
		.await
		.map_err(|e| AppError::InternalServerError(e.to_string()))?;

	Ok(models.into_iter().map(model_to_entity).collect())
}

pub async fn find_by_mentee_id(
	db: &Arc<DatabaseConnection>,
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
		.all(db.as_ref())
		.await
		.map_err(|e| AppError::InternalServerError(e.to_string()))?;

	Ok(models.into_iter().map(model_to_entity).collect())
}

pub async fn find_booked_dates(
	db: &Arc<DatabaseConnection>,
	mentor_id: Uuid,
) -> Result<Vec<String>, AppError> {
	let sessions = SessionsEntity::find()
		.filter(SessionColumn::MentorId.eq(mentor_id))
		.filter(SessionColumn::Status.is_in(["pending", "confirmed"]))
		.order_by(SessionColumn::ScheduledAt, Order::Asc)
		.all(db.as_ref())
		.await
		.map_err(|e| AppError::InternalServerError(e.to_string()))?;

	Ok(
		sessions
			.into_iter()
			.map(|s| s.scheduled_at.to_rfc3339())
			.collect(),
	)
}

pub async fn count_by_mentor(
	db: &Arc<DatabaseConnection>,
	mentor_id: Uuid,
	status_filter: Option<String>,
) -> Result<usize, AppError> {
	let mut query =
		SessionsEntity::find().filter(SessionColumn::MentorId.eq(mentor_id));

	if let Some(status) = status_filter {
		query = query.filter(SessionColumn::Status.eq(status));
	}

	let count = query
		.count(db.as_ref())
		.await
		.map_err(|e| AppError::InternalServerError(e.to_string()))?;

	Ok(count as usize)
}

pub async fn count_by_mentee(
	db: &Arc<DatabaseConnection>,
	mentee_id: Uuid,
	status_filter: Option<String>,
) -> Result<usize, AppError> {
	let mut query =
		SessionsEntity::find().filter(SessionColumn::MenteeId.eq(mentee_id));

	if let Some(status) = status_filter {
		query = query.filter(SessionColumn::Status.eq(status));
	}

	let count = query
		.count(db.as_ref())
		.await
		.map_err(|e| AppError::InternalServerError(e.to_string()))?;

	Ok(count as usize)
}

pub async fn find_all_paginated(
	db: &Arc<DatabaseConnection>,
	params: PaginationParams,
) -> Result<PaginatorResponse<SessionEntity>, AppError> {
	let page = params.page.max(1);
	let per_page = params.per_page.clamp(1, 100);

	let query = match params.sort_direction {
		Some(SortDirection::Asc) => {
			SessionsEntity::find().order_by(SessionColumn::CreatedAt, Order::Asc)
		}
		_ => SessionsEntity::find().order_by(SessionColumn::CreatedAt, Order::Desc),
	};

	let paginator = query.paginate(db.as_ref(), per_page as u64);
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
