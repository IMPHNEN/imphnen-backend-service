use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, Order, QueryOrder, PaginatorTrait};
use paginator_rs::{PaginationParams, SortDirection};
use paginator_utils::{PaginatorResponse, PaginatorResponseMeta};
use uuid::Uuid;
use imphnen_utils::AppError;
use imphnen_entities::seaorm::auth::mentors::{
    Entity as MentorsEntity,
    Column as MentorColumn,
    ActiveModel as MentorActiveModel,
    Model as MentorModel,
};
use crate::mentors::domain::{mentor::MentorEntity, repository::MentorRepository};

fn model_to_entity(model: MentorModel) -> MentorEntity {
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
            model.topics_of_interest.unwrap_or(serde_json::Value::Array(vec![])),
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
    db: Arc<DatabaseConnection>,
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
        let page = params.page.max(1);
        let per_page = params.per_page.clamp(1, 100);

        let mut query = MentorsEntity::find()
            .filter(MentorColumn::IsDeleted.eq(false));

        query = match params.sort_by.as_deref() {
            Some("updated_at") => match params.sort_direction {
                Some(SortDirection::Asc) => query.order_by(MentorColumn::UpdatedAt, Order::Asc),
                _ => query.order_by(MentorColumn::UpdatedAt, Order::Desc),
            },
            _ => match params.sort_direction {
                Some(SortDirection::Asc) => query.order_by(MentorColumn::CreatedAt, Order::Asc),
                _ => query.order_by(MentorColumn::CreatedAt, Order::Desc),
            },
        };

        let paginator = query.paginate(self.db.as_ref(), per_page as u64);
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
        let mut query = MentorsEntity::find()
            .filter(MentorColumn::UserId.eq(user_id));

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
            availability_commitment: ActiveValue::Set(Some(entity.availability_commitment)),
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

        if !entity.industries.is_empty() {
            active_model.industries = ActiveValue::Set(Some(
                serde_json::to_value(&entity.industries)
                    .map_err(|e| AppError::InternalServerError(e.to_string()))?,
            ));
        }
        if !entity.expertise.is_empty() {
            active_model.expertise = ActiveValue::Set(Some(
                serde_json::to_value(&entity.expertise)
                    .map_err(|e| AppError::InternalServerError(e.to_string()))?,
            ));
        }
        if !entity.languages.is_empty() {
            active_model.languages = ActiveValue::Set(Some(
                serde_json::to_value(&entity.languages)
                    .map_err(|e| AppError::InternalServerError(e.to_string()))?,
            ));
        }
        if !entity.current_company.is_empty() {
            active_model.current_company = ActiveValue::Set(Some(entity.current_company));
        }
        if !entity.current_role.is_empty() {
            active_model.current_role = ActiveValue::Set(Some(entity.current_role));
        }
        active_model.years_of_experience = ActiveValue::Set(Some(entity.years_of_experience));
        if !entity.topics_of_interest.is_empty() {
            active_model.topics_of_interest = ActiveValue::Set(Some(
                serde_json::to_value(&entity.topics_of_interest)
                    .map_err(|e| AppError::InternalServerError(e.to_string()))?,
            ));
        }
        if !entity.preferred_mentee_level.is_empty() {
            active_model.preferred_mentee_level = ActiveValue::Set(Some(
                serde_json::to_string(&entity.preferred_mentee_level)
                    .map_err(|e| AppError::InternalServerError(e.to_string()))?,
            ));
        }
        if !entity.preferred_mentoring_formats.is_empty() {
            active_model.preferred_mentoring_formats = ActiveValue::Set(Some(
                serde_json::to_value(&entity.preferred_mentoring_formats)
                    .map_err(|e| AppError::InternalServerError(e.to_string()))?,
            ));
        }
        if !entity.availability_commitment.is_empty() {
            active_model.availability_commitment =
                ActiveValue::Set(Some(entity.availability_commitment));
        }
        active_model.mentoring_rate = ActiveValue::Set(Some(entity.mentoring_rate));
        if !entity.status.is_empty() {
            active_model.status = ActiveValue::Set(Some(entity.status));
        }
        active_model.updated_at = ActiveValue::Set(chrono::Utc::now());

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
