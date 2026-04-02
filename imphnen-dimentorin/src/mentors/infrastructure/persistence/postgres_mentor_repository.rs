use super::postgres_mentor_queries;
use super::postgres_mentor_write::apply_entity_to_model;
use crate::mentors::domain::{mentor::MentorEntity, repository::MentorRepository};
use async_trait::async_trait;
use imphnen_entities::seaorm::auth::mentors::{
	ActiveModel as MentorActiveModel, Column as MentorColumn, Entity as MentorsEntity,
	Model as MentorModel,
};
use imphnen_utils::AppError;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use sea_orm::ActiveValue;
use sea_orm::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

pub fn model_to_entity(model: MentorModel) -> MentorEntity {
	MentorEntity {
		id: model.id,
		user_id: model.user_id,
		industries: serde_json::from_value(
			model.industries.unwrap_or(serde_json::Value::Array(vec![])),
		)
		.unwrap_or_default(),
		expertise: serde_json::from_value(
			model.expertise.unwrap_or(serde_json::Value::Array(vec![])),
		)
		.unwrap_or_default(),
		languages: serde_json::from_value(
			model.languages.unwrap_or(serde_json::Value::Array(vec![])),
		)
		.unwrap_or_default(),
		current_company: model.current_company.unwrap_or_default(),
		current_role: model.current_role.unwrap_or_default(),
		years_of_experience: model.years_of_experience.unwrap_or(0),
		topics_of_interest: serde_json::from_value(
			model
				.topics_of_interest
				.unwrap_or(serde_json::Value::Array(vec![])),
		)
		.unwrap_or_default(),
		preferred_mentee_level: serde_json::from_str(
			&model.preferred_mentee_level.unwrap_or_default(),
		)
		.unwrap_or_default(),
		preferred_mentoring_formats: serde_json::from_value(
			model
				.preferred_mentoring_formats
				.unwrap_or(serde_json::Value::Array(vec![])),
		)
		.unwrap_or_default(),
		availability_commitment: model.availability_commitment.unwrap_or_default(),
		mentoring_rate: model.mentoring_rate.unwrap_or(0.0),
		status: model.status.unwrap_or_default(),
		is_deleted: model.is_deleted,
		created_at: model.created_at,
		updated_at: model.updated_at,
	}
}

pub struct PostgresMentorRepository {
	pub db: Arc<DatabaseConnection>,
}

impl PostgresMentorRepository {
	pub fn new(db: DatabaseConnection) -> Self {
		Self { db: Arc::new(db) }
	}
}

#[async_trait]
impl MentorRepository for PostgresMentorRepository {
	async fn find_all(
		&self,
		params: PaginationParams,
	) -> Result<PaginatorResponse<MentorEntity>, AppError> {
		postgres_mentor_queries::find_all_paginated(&self.db, params).await
	}

	async fn find_by_id(
		&self,
		id: Uuid,
		include_deleted: bool,
	) -> Result<MentorEntity, AppError> {
		let mut query = MentorsEntity::find_by_id(id);
		if !include_deleted {
			query = query.filter(MentorColumn::IsDeleted.eq(false));
		}
		let model = query
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?
			.ok_or_else(|| AppError::NotFoundError("Mentor not found".to_string()))?;
		Ok(model_to_entity(model))
	}

	async fn find_by_user_id(
		&self,
		user_id: Uuid,
		include_deleted: bool,
	) -> Result<MentorEntity, AppError> {
		postgres_mentor_queries::find_by_user_id(&self.db, user_id, include_deleted)
			.await
	}

	async fn create(&self, entity: MentorEntity) -> Result<Uuid, AppError> {
		let active_model = MentorActiveModel {
			user_id: ActiveValue::Set(entity.user_id),
			industries: ActiveValue::Set(Some(
				serde_json::to_value(&entity.industries)
					.map_err(|e| AppError::InternalServerError(e.to_string()))?,
			)),
			expertise: ActiveValue::Set(Some(
				serde_json::to_value(&entity.expertise)
					.map_err(|e| AppError::InternalServerError(e.to_string()))?,
			)),
			languages: ActiveValue::Set(Some(
				serde_json::to_value(&entity.languages)
					.map_err(|e| AppError::InternalServerError(e.to_string()))?,
			)),
			current_company: ActiveValue::Set(Some(entity.current_company)),
			current_role: ActiveValue::Set(Some(entity.current_role)),
			years_of_experience: ActiveValue::Set(Some(entity.years_of_experience)),
			topics_of_interest: ActiveValue::Set(Some(
				serde_json::to_value(&entity.topics_of_interest)
					.map_err(|e| AppError::InternalServerError(e.to_string()))?,
			)),
			preferred_mentee_level: ActiveValue::Set(Some(
				serde_json::to_string(&entity.preferred_mentee_level)
					.map_err(|e| AppError::InternalServerError(e.to_string()))?,
			)),
			preferred_mentoring_formats: ActiveValue::Set(Some(
				serde_json::to_value(&entity.preferred_mentoring_formats)
					.map_err(|e| AppError::InternalServerError(e.to_string()))?,
			)),
			availability_commitment: ActiveValue::Set(Some(
				entity.availability_commitment,
			)),
			mentoring_rate: ActiveValue::Set(Some(entity.mentoring_rate)),
			status: ActiveValue::Set(Some(entity.status)),
			is_deleted: ActiveValue::Set(false),
			created_at: ActiveValue::Set(chrono::Utc::now()),
			updated_at: ActiveValue::Set(chrono::Utc::now()),
			..Default::default()
		};
		let result = MentorsEntity::insert(active_model)
			.exec(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(result.last_insert_id)
	}

	async fn update(&self, entity: MentorEntity) -> Result<(), AppError> {
		let mut active_model: MentorActiveModel = MentorsEntity::find_by_id(entity.id)
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?
			.ok_or_else(|| AppError::NotFoundError("Mentor not found".to_string()))?
			.into();
		apply_entity_to_model(&entity, &mut active_model)?;
		active_model
			.update(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(())
	}

	async fn soft_delete(&self, id: Uuid) -> Result<(), AppError> {
		let model = MentorsEntity::find_by_id(id)
			.filter(MentorColumn::IsDeleted.eq(false))
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?
			.ok_or_else(|| AppError::NotFoundError("Mentor not found".to_string()))?;
		let mut active_model: MentorActiveModel = model.into();
		active_model.is_deleted = ActiveValue::Set(true);
		active_model.updated_at = ActiveValue::Set(chrono::Utc::now());
		active_model
			.update(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(())
	}
}
