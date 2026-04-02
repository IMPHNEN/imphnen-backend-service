#![allow(clippy::field_reassign_with_default)]
use super::postgres_user_queries::{
	build_role_dto, build_user_dto, query_user_list,
};
use crate::users::domain::{UserEntity, UserListItem, UserRepository};
use async_trait::async_trait;
use chrono::Utc;
use imphnen_entities::{
	UsersDetailQueryDto,
	seaorm::auth::roles::Entity as RolesEntity,
	seaorm::auth::users::{
		ActiveModel as UserActiveModel, Column as UserColumn, Entity as UsersEntity,
	},
};
use imphnen_utils::AppError;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use sea_orm::ActiveValue;
use sea_orm::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

fn user_detail_to_entity(dto: UsersDetailQueryDto) -> UserEntity {
	UserEntity {
		id: dto.id,
		email: dto.email,
		fullname: dto.fullname,
		legal_name: dto.legal_name,
		password: dto.password,
		avatar: dto.avatar,
		is_active: dto.is_active,
		is_deleted: dto.is_deleted,
		role: dto.role,
		profile_extension: dto.profile_extension,
		created_at: dto.created_at,
		updated_at: dto.updated_at,
		mentor_id: dto.mentor_id,
	}
}

pub struct PostgresUserRepository {
	db: Arc<DatabaseConnection>,
}

impl PostgresUserRepository {
	pub fn new(db: DatabaseConnection) -> Self {
		Self { db: Arc::new(db) }
	}
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
	async fn find_all(
		&self,
		params: PaginationParams,
	) -> Result<PaginatorResponse<UserListItem>, AppError> {
		query_user_list(&self.db, params).await
	}

	async fn find_by_id(&self, id: &str) -> Result<UserEntity, AppError> {
		let user_id = Uuid::parse_str(id)
			.map_err(|_| AppError::BadRequestError("Invalid user ID".into()))?;
		let (user, role) = UsersEntity::find_by_id(user_id)
			.filter(UserColumn::DeletedAt.is_null())
			.find_also_related(RolesEntity)
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?
			.ok_or_else(|| AppError::NotFoundError("User not found in database".into()))?;
		Ok(user_detail_to_entity(
			build_user_dto(user, build_role_dto(role)).from_profile_extension(),
		))
	}

	async fn find_by_email(&self, email: String) -> Result<UserEntity, AppError> {
		let (user, role) = UsersEntity::find()
			.filter(UserColumn::Email.eq(&email))
			.filter(UserColumn::DeletedAt.is_null())
			.find_also_related(RolesEntity)
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?
			.ok_or_else(|| AppError::NotFoundError("User not found".into()))?;
		Ok(user_detail_to_entity(
			build_user_dto(user, build_role_dto(role)).from_profile_extension(),
		))
	}

	async fn create(&self, entity: UserEntity) -> Result<String, AppError> {
		let existing = UsersEntity::find()
			.filter(UserColumn::Email.eq(entity.email.clone()))
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		if existing.is_some() {
			return Err(AppError::ConflictError(
				"User with this email already exists".into(),
			));
		}
		let full_name = entity.fullname.clone();
		let (first_name, last_name) =
			full_name.split_once(' ').unwrap_or((&full_name, ""));
		let role_id = entity
			.role
			.id
			.parse::<Uuid>()
			.ok()
			.or_else(|| entity.role.id.is_empty().then_some(Uuid::nil()));
		let active_model = UserActiveModel {
			id: ActiveValue::Set(Uuid::new_v4()),
			email: ActiveValue::Set(entity.email.clone()),
			password_hash: ActiveValue::Set(entity.password),
			username: ActiveValue::Set(entity.email.clone()),
			first_name: ActiveValue::Set(Some(first_name.to_string())),
			last_name: ActiveValue::Set(Some(last_name.to_string())),
			avatar_url: ActiveValue::Set(entity.avatar),
			is_verified: ActiveValue::Set(false),
			is_active: ActiveValue::Set(entity.is_active),
			metadata: ActiveValue::Set(
				entity
					.profile_extension
					.map(|p| serde_json::to_value(p).unwrap_or_default()),
			),
			created_at: ActiveValue::Set(Utc::now()),
			updated_at: ActiveValue::Set(Utc::now()),
			deleted_at: ActiveValue::Set(None),
			role_id: ActiveValue::Set(role_id.filter(|id| !id.is_nil())),
		};
		UsersEntity::insert(active_model)
			.exec(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok("Successfully created user".into())
	}

	async fn update(&self, entity: UserEntity) -> Result<String, AppError> {
		let user_id = Uuid::parse_str(&entity.id)
			.map_err(|_| AppError::BadRequestError("Invalid user ID".into()))?;
		let mut active_model: UserActiveModel = UsersEntity::find_by_id(user_id)
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?
			.ok_or_else(|| AppError::NotFoundError("User not found".into()))?
			.into();
		let full_name = entity.fullname.clone();
		let (first_name, last_name) =
			full_name.split_once(' ').unwrap_or((&full_name, ""));
		active_model.email = ActiveValue::Set(entity.email);
		active_model.first_name = ActiveValue::Set(Some(first_name.to_string()));
		active_model.last_name = ActiveValue::Set(Some(last_name.to_string()));
		active_model.avatar_url = ActiveValue::Set(entity.avatar);
		active_model.is_active = ActiveValue::Set(entity.is_active);
		active_model.updated_at = ActiveValue::Set(Utc::now());
		if !entity.password.is_empty() {
			active_model.password_hash = ActiveValue::Set(entity.password);
		}
		let role_id = entity.role.id.parse::<Uuid>().ok();
		if role_id.is_some() {
			active_model.role_id = ActiveValue::Set(role_id);
		}
		if entity.profile_extension.is_some() {
			active_model.metadata = ActiveValue::Set(
				entity
					.profile_extension
					.map(|p| serde_json::to_value(p).unwrap_or_default()),
			);
		}
		active_model
			.update(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok("Success update user".into())
	}

	async fn delete(&self, id: String) -> Result<String, AppError> {
		let user_id = Uuid::parse_str(&id)
			.map_err(|_| AppError::BadRequestError("Invalid user ID".into()))?;
		let mut active_model: UserActiveModel = UsersEntity::find_by_id(user_id)
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?
			.ok_or_else(|| AppError::NotFoundError("User not found".into()))?
			.into();
		active_model.deleted_at = ActiveValue::Set(Some(Utc::now()));
		active_model.updated_at = ActiveValue::Set(Utc::now());
		active_model
			.update(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok("Success delete user".into())
	}
}
