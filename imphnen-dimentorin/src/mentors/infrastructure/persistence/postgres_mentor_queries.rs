use super::postgres_mentor_repository::model_to_entity;
use crate::mentors::domain::mentor::MentorEntity;
use imphnen_entities::seaorm::auth::mentors::{
	Column as MentorColumn, Entity as MentorsEntity,
};
use imphnen_utils::AppError;
use paginator_rs::{PaginationParams, SortDirection};
use paginator_utils::{PaginatorResponse, PaginatorResponseMeta};
use sea_orm::prelude::*;
use sea_orm::{Order, PaginatorTrait, QueryOrder};
use std::sync::Arc;

pub async fn find_all_paginated(
	db: &Arc<DatabaseConnection>,
	params: PaginationParams,
) -> Result<PaginatorResponse<MentorEntity>, AppError> {
	let page = params.page.max(1);
	let per_page = params.per_page.clamp(1, 100);

	let mut query = MentorsEntity::find().filter(MentorColumn::IsDeleted.eq(false));

	query = match params.sort_by.as_deref() {
		Some("updated_at") => match params.sort_direction {
			Some(SortDirection::Asc) => {
				query.order_by(MentorColumn::UpdatedAt, Order::Asc)
			}
			_ => query.order_by(MentorColumn::UpdatedAt, Order::Desc),
		},
		_ => match params.sort_direction {
			Some(SortDirection::Asc) => {
				query.order_by(MentorColumn::CreatedAt, Order::Asc)
			}
			_ => query.order_by(MentorColumn::CreatedAt, Order::Desc),
		},
	};

	let paginator = query.paginate(db.as_ref(), per_page as u64);
	let total = paginator
		.num_items()
		.await
		.map_err(|e| AppError::InternalServerError(e.to_string()))?;
	let mentors = paginator
		.fetch_page((page - 1) as u64)
		.await
		.map_err(|e| AppError::InternalServerError(e.to_string()))?;

	let data = mentors.into_iter().map(model_to_entity).collect();
	let meta = PaginatorResponseMeta::new(page, per_page, total as u32);
	Ok(PaginatorResponse { data, meta })
}

pub async fn find_by_user_id(
	db: &Arc<DatabaseConnection>,
	user_id: Uuid,
	include_deleted: bool,
) -> Result<MentorEntity, AppError> {
	let mut query = MentorsEntity::find().filter(MentorColumn::UserId.eq(user_id));

	if !include_deleted {
		query = query.filter(MentorColumn::IsDeleted.eq(false));
	}

	let model = query
		.one(db.as_ref())
		.await
		.map_err(|e| AppError::InternalServerError(e.to_string()))?
		.ok_or_else(|| AppError::NotFoundError("Mentor not found".to_string()))?;

	Ok(model_to_entity(model))
}
