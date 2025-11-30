use crate::v1::mentors::mentors_schema::MentorSchema;
use imphnen_libs::AppState;
use imphnen_libs::AppStatePostgresExt;
use imphnen_entities::seaorm::auth::mentors::{Entity as Mentors, ActiveModel as MentorActiveModel, Column as MentorColumn};
use imphnen_entities::seaorm::auth::users::Entity as Users;
use anyhow::{Result, bail};
use sea_orm::*;
use sea_orm::EntityTrait;
use uuid::Uuid;
use sea_orm::ActiveModelTrait as ActiveModelTraitSpecific;
use anyhow::anyhow;
use imphnen_entities::{MetaRequestDto, ResponseListSuccessDto};
use imphnen_utils::Result as UtilsResult;
use crate::v1::mentors::mentors_dto::MentorDetailQueryDto;
use std::time::Instant;
use tracing::{instrument, info};
use serde_json;
use chrono::Utc;

pub struct MentorsRepository<'a> {
    state: &'a AppState,
}

impl<'a> MentorsRepository<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }

    fn get_db(&self) -> &DatabaseConnection {
        self.state.postgres_db()
    }

    #[instrument(skip(self, data), err)]
    pub async fn query_create_mentor(&self, data: MentorSchema) -> Result<String> {
        let now = Instant::now();
        
        let mentor_active_model = MentorActiveModel {
            user_id: ActiveValue::Set(Uuid::parse_str(&data.user_id.unwrap_or_default())?),
            industries: ActiveValue::Set(Some(data.industries.into())),
            expertise: ActiveValue::Set(Some(data.expertise.into())),
            languages: ActiveValue::Set(Some(data.languages.into())),
            current_company: ActiveValue::Set(Some(data.current_company)),
            current_role: ActiveValue::Set(Some(data.current_role)),
            years_of_experience: ActiveValue::Set(Some(data.years_of_experience)),
            topics_of_interest: ActiveValue::Set(Some(data.topics_of_interest.into())),
            preferred_mentee_level: ActiveValue::Set(Some(serde_json::to_string(&data.preferred_mentee_level).unwrap())),
            preferred_mentoring_formats: ActiveValue::Set(Some(data.preferred_mentoring_formats.into())),
            availability_commitment: ActiveValue::Set(Some(data.availability_commitment)),
            mentoring_rate: ActiveValue::Set(Some(data.mentoring_rate)),
            status: ActiveValue::Set(Some(data.status)),
            created_at: ActiveValue::Set(Utc::now()),
            updated_at: ActiveValue::Set(Utc::now()),
            ..Default::default()
        };

        info!("Executing PostgreSQL create in query_create_mentor");
        let result = <MentorActiveModel as sea_orm::ActiveModelTrait>::insert(mentor_active_model, self.get_db()).await?;
        
        let elapsed = now.elapsed();
        if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()) == "development" {
            println!("Query 'query_create_mentor' took: {elapsed:.2?}");
        }

        Ok(result.id.to_string())
    }

    #[instrument(skip(self, data), err)]
    pub async fn query_update_mentor(&self, data: MentorSchema) -> Result<String> {
        let now = Instant::now();

        let mentor_id = Uuid::parse_str(&data.id)?;
        let existing_mentor = Mentors::find_by_id(mentor_id)
            .one(self.get_db())
            .await?
            .ok_or_else(|| anyhow!("Mentor not found"))?;

        let mut mentor_active_model: MentorActiveModel = existing_mentor.into();

        if !data.industries.is_empty() { mentor_active_model.industries = ActiveValue::Set(Some(serde_json::to_value(data.industries).unwrap())); }
        if !data.expertise.is_empty() { mentor_active_model.expertise = ActiveValue::Set(Some(serde_json::to_value(data.expertise).unwrap())); }
        if !data.languages.is_empty() { mentor_active_model.languages = ActiveValue::Set(Some(serde_json::to_value(data.languages).unwrap())); }
        if !data.current_company.is_empty() { mentor_active_model.current_company = ActiveValue::Set(Some(data.current_company)); }
        if !data.current_role.is_empty() { mentor_active_model.current_role = ActiveValue::Set(Some(data.current_role)); }
        mentor_active_model.years_of_experience = ActiveValue::Set(Some(data.years_of_experience));
        if !data.topics_of_interest.is_empty() { mentor_active_model.topics_of_interest = ActiveValue::Set(Some(serde_json::to_value(data.topics_of_interest).unwrap())); }
        if !data.preferred_mentee_level.is_empty() { mentor_active_model.preferred_mentee_level = ActiveValue::Set(Some(serde_json::to_string(&data.preferred_mentee_level).unwrap())); }
        if !data.preferred_mentoring_formats.is_empty() { mentor_active_model.preferred_mentoring_formats = ActiveValue::Set(Some(serde_json::to_value(data.preferred_mentoring_formats).unwrap())); }
        if !data.availability_commitment.is_empty() { mentor_active_model.availability_commitment = ActiveValue::Set(Some(data.availability_commitment)); }
        mentor_active_model.mentoring_rate = ActiveValue::Set(Some(data.mentoring_rate));
        if !data.status.is_empty() { mentor_active_model.status = ActiveValue::Set(Some(data.status)); }
        mentor_active_model.updated_at = ActiveValue::Set(Utc::now());

        info!("Executing PostgreSQL update in query_update_mentor");
        let result = mentor_active_model.update(self.get_db()).await?;
        
        let elapsed = now.elapsed();
        if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()) == "development" {
            println!("Query 'query_update_mentor' took: {elapsed:.2?}");
        }

        Ok(format!("Success update mentor: {}", result.id))
    }

    #[instrument(skip(self, id), err)]
    pub async fn query_delete_mentor(&self, id: &str) -> Result<String> {
        let now = Instant::now();

        let mentor = Mentors::find_by_id(Uuid::parse_str(id)?)
            .one(self.get_db())
            .await?
            .ok_or_else(|| anyhow!("Mentor not found"))?;

        if mentor.is_deleted {
            bail!("Mentor is already soft deleted");
        }

        let mut mentor_active_model: MentorActiveModel = mentor.into();
        mentor_active_model.is_deleted = ActiveValue::Set(true);
        mentor_active_model.updated_at = ActiveValue::Set(Utc::now());

        info!("Executing PostgreSQL soft delete in query_delete_mentor");
        let result = mentor_active_model.update(self.get_db()).await?;
        
        let elapsed = now.elapsed();
        if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()) == "development" {
            println!("Query 'query_delete_mentor' took: {elapsed:.2?}");
        }

        Ok(format!("Success soft delete mentor: {}", result.id))
    }

    #[instrument(skip(self, email, include_deleted), err)]
    pub async fn query_mentor_by_email(
        &self,
        email: String,
        include_deleted: bool,
    ) -> UtilsResult<MentorDetailQueryDto> {
        let now = Instant::now();
        let db = self.get_db();

        let mut query = Mentors::find()
            .find_also_related(Users);

        if !include_deleted {
            query = query.filter(MentorColumn::IsDeleted.eq(false));
        }

        let (mentor, user) = query
            .filter(imphnen_entities::seaorm::auth::users::Column::Email.eq(email))
            .one(db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Mentor not found"))?;

        let _user = user.ok_or_else(|| anyhow::anyhow!("User not found for mentor"))?;

        let result = MentorDetailQueryDto {
            id: mentor.id.to_string(),
            user_id: mentor.user_id.to_string(),
            industries: serde_json::from_value(mentor.industries.clone().unwrap_or(serde_json::Value::Null)).unwrap_or_default(),
            expertise: serde_json::from_value(mentor.expertise.clone().unwrap_or(serde_json::Value::Null)).unwrap_or_default(),
            languages: serde_json::from_value(mentor.languages.clone().unwrap_or(serde_json::Value::Null)).unwrap_or_default(),
            current_company: mentor.current_company.unwrap_or_default(),
            current_role: mentor.current_role.unwrap_or_default(),
            years_of_experience: mentor.years_of_experience.unwrap_or(0),
            topics_of_interest: serde_json::from_value(mentor.topics_of_interest.clone().unwrap_or(serde_json::Value::Null)).unwrap_or_default(),
            preferred_mentee_level: serde_json::from_str(&mentor.preferred_mentee_level.unwrap_or_default()).unwrap_or_default(),
            preferred_mentoring_formats: serde_json::from_value(mentor.preferred_mentoring_formats.clone().unwrap_or(serde_json::Value::Null)).unwrap_or_default(),
            availability_commitment: mentor.availability_commitment.unwrap_or_default(),
            mentoring_rate: mentor.mentoring_rate.unwrap_or(0.0),
            status: mentor.status.unwrap_or_default(),
            is_deleted: mentor.is_deleted,
            created_at: mentor.created_at.to_rfc3339(),
            updated_at: mentor.updated_at.to_rfc3339(),
        };

        let elapsed = now.elapsed();
        if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()) == "development" {
            println!("Query 'query_mentor_by_email' took: {elapsed:.2?}");
        }

        Ok(result)
    }

    #[instrument(skip(self, meta), err)]
    pub async fn query_mentor_list(
        &self,
        meta: MetaRequestDto,
    ) -> UtilsResult<ResponseListSuccessDto<Vec<MentorDetailQueryDto>>> {
        let now = Instant::now();
        let db = self.get_db();

        let page = meta.page.unwrap_or(1);
        let per_page = meta.per_page.unwrap_or(10);

        let mut query = Mentors::find()
            .filter(MentorColumn::IsDeleted.eq(false))
            .find_also_related(Users);

        // Apply sorting
        if let Some(sort_by) = &meta.sort_by {
            let order = meta.order.as_deref().unwrap_or("asc");
            match sort_by.as_str() {
                "created_at" => {
                    if order == "desc" {
                        query = query.order_by_desc(MentorColumn::CreatedAt);
                    } else {
                        query = query.order_by_asc(MentorColumn::CreatedAt);
                    }
                }
                "updated_at" => {
                    if order == "desc" {
                        query = query.order_by_desc(MentorColumn::UpdatedAt);
                    } else {
                        query = query.order_by_asc(MentorColumn::UpdatedAt);
                    }
                }
                _ => {
                    query = query.order_by_desc(MentorColumn::CreatedAt);
                }
            }
        } else {
            query = query.order_by_desc(MentorColumn::CreatedAt);
        }

        let paginator = query.paginate(db, per_page);
        let total_pages = paginator.num_pages().await?;
        let mentors = paginator.fetch_page(page - 1).await?;

        let data: Vec<MentorDetailQueryDto> = mentors
            .into_iter()
            .filter_map(|(mentor, user)| {
                user.map(|_u| MentorDetailQueryDto {
                    id: mentor.id.to_string(),
                    user_id: mentor.user_id.to_string(),
                    industries: serde_json::from_value(mentor.industries.clone().unwrap_or(serde_json::Value::Null)).unwrap_or_default(),
                    expertise: serde_json::from_value(mentor.expertise.clone().unwrap_or(serde_json::Value::Null)).unwrap_or_default(),
                    languages: serde_json::from_value(mentor.languages.clone().unwrap_or(serde_json::Value::Null)).unwrap_or_default(),
                    current_company: mentor.current_company.unwrap_or_default(),
                    current_role: mentor.current_role.unwrap_or_default(),
                    years_of_experience: mentor.years_of_experience.unwrap_or(0),
                    topics_of_interest: serde_json::from_value(mentor.topics_of_interest.clone().unwrap_or(serde_json::Value::Null)).unwrap_or_default(),
                    preferred_mentee_level: serde_json::from_str(&mentor.preferred_mentee_level.unwrap_or_default()).unwrap_or_default(),
                    preferred_mentoring_formats: serde_json::from_value(mentor.preferred_mentoring_formats.clone().unwrap_or(serde_json::Value::Null)).unwrap_or_default(),
                    availability_commitment: mentor.availability_commitment.unwrap_or_default(),
                    mentoring_rate: mentor.mentoring_rate.unwrap_or(0.0),
                    status: mentor.status.unwrap_or_default(),
                    is_deleted: mentor.is_deleted,
                    created_at: mentor.created_at.to_rfc3339(),
                    updated_at: mentor.updated_at.to_rfc3339(),
                })
            })
            .collect();

        let elapsed = now.elapsed();
        if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()) == "development" {
            println!("Query 'query_mentor_list' took: {elapsed:.2?}");
        }

        let response = ResponseListSuccessDto {
            data,
            meta: Some(imphnen_entities::MetaResponseDto {
                page: Some(page),
                per_page: Some(per_page),
                total: Some(total_pages),
            }),
        };

        Ok(response)
    }

    #[instrument(skip(self, id, include_deleted), err)]
    pub async fn query_mentor_by_id(
        &self,
        id: &str,
        include_deleted: bool,
    ) -> UtilsResult<MentorDetailQueryDto> {
        let now = Instant::now();
        let db = self.get_db();

        let mentor_id = Uuid::parse_str(id).map_err(|e| anyhow!("Invalid mentor ID: {}", e))?;

        let mut query = Mentors::find_by_id(mentor_id)
            .find_also_related(Users);

        if !include_deleted {
            query = query.filter(MentorColumn::IsDeleted.eq(false));
        }

        let (mentor, user) = query
            .one(db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Mentor not found"))?;

        let _user = user.ok_or_else(|| anyhow::anyhow!("User not found for mentor"))?;

        let result = MentorDetailQueryDto {
            id: mentor.id.to_string(),
            user_id: mentor.user_id.to_string(),
            industries: serde_json::from_value(mentor.industries.clone().unwrap_or(serde_json::Value::Null)).unwrap_or_default(),
            expertise: serde_json::from_value(mentor.expertise.clone().unwrap_or(serde_json::Value::Null)).unwrap_or_default(),
            languages: serde_json::from_value(mentor.languages.clone().unwrap_or(serde_json::Value::Null)).unwrap_or_default(),
            current_company: mentor.current_company.unwrap_or_default(),
            current_role: mentor.current_role.unwrap_or_default(),
            years_of_experience: mentor.years_of_experience.unwrap_or(0),
            topics_of_interest: serde_json::from_value(mentor.topics_of_interest.clone().unwrap_or(serde_json::Value::Null)).unwrap_or_default(),
            preferred_mentee_level: serde_json::from_str(&mentor.preferred_mentee_level.unwrap_or_default()).unwrap_or_default(),
            preferred_mentoring_formats: serde_json::from_value(mentor.preferred_mentoring_formats.clone().unwrap_or(serde_json::Value::Null)).unwrap_or_default(),
            availability_commitment: mentor.availability_commitment.unwrap_or_default(),
            mentoring_rate: mentor.mentoring_rate.unwrap_or(0.0),
            status: mentor.status.unwrap_or_default(),
            is_deleted: mentor.is_deleted,
            created_at: mentor.created_at.to_rfc3339(),
            updated_at: mentor.updated_at.to_rfc3339(),
        };

        let elapsed = now.elapsed();
        if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()) == "development" {
            println!("Query 'query_mentor_by_id' took: {elapsed:.2?}");
        }

        Ok(result)
    }
}
