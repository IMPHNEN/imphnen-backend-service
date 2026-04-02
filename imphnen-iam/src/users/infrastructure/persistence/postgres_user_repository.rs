#![allow(clippy::field_reassign_with_default)]
use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, Order, QueryOrder, PaginatorTrait};
use paginator_rs::{PaginationParams, SortDirection};
use paginator_utils::{PaginatorResponse, PaginatorResponseMeta};
use uuid::Uuid;
use chrono::Utc;
use imphnen_utils::AppError;
use imphnen_entities::{
    UsersDetailQueryDto, RolesDetailQueryDto, PermissionsQueryDto,
    seaorm::auth::users::{Entity as UsersEntity, ActiveModel as UserActiveModel, Column as UserColumn},
    seaorm::auth::roles::Entity as RolesEntity,
};
use crate::users::domain::{UserEntity, UserListItem, UserRepository};

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

fn build_role_dto(role: Option<imphnen_entities::seaorm::auth::roles::Model>) -> RolesDetailQueryDto {
    role.map_or_else(RolesDetailQueryDto::default, |r| RolesDetailQueryDto {
        id: r.id.to_string(),
        name: r.name,
        permissions: r.permissions.clone().and_then(|json| {
            serde_json::from_value::<Vec<String>>(json).ok().map(|list| {
                list.into_iter().map(|p| Some(PermissionsQueryDto {
                    id: Some(p.clone()),
                    name: Some(p),
                    created_at: None,
                    updated_at: None,
                })).collect()
            })
        }),
        is_deleted: r.deleted_at.is_some(),
        created_at: Some(r.created_at.to_rfc3339()),
        updated_at: Some(r.updated_at.to_rfc3339()),
    })
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
    async fn find_all(&self, params: PaginationParams) -> Result<PaginatorResponse<UserListItem>, AppError> {
        let page = params.page.max(1);
        let per_page = params.per_page.clamp(1, 100);

        let mut query = UsersEntity::find()
            .filter(UserColumn::DeletedAt.is_null())
            .filter(UserColumn::IsActive.eq(true));

        if let Some(ref search) = params.search {
            query = query.filter(
                UserColumn::Email.contains(&search.query)
                    .or(UserColumn::FirstName.contains(&search.query))
                    .or(UserColumn::LastName.contains(&search.query))
            );
        }

        let order = match params.sort_direction {
            Some(SortDirection::Desc) => Order::Desc,
            _ => Order::Asc,
        };
        query = match params.sort_by.as_deref() {
            Some("email") => query.order_by(UserColumn::Email, order),
            _ => query.order_by(UserColumn::CreatedAt, order),
        };

        let paginator = query.paginate(self.db.as_ref(), per_page as u64);
        let users = paginator.fetch_page((page - 1) as u64).await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let role_ids: Vec<Uuid> = users.iter().filter_map(|u| u.role_id).collect();
        let roles = if !role_ids.is_empty() {
            RolesEntity::find()
                .filter(imphnen_entities::seaorm::auth::roles::Column::Id.is_in(role_ids))
                .all(self.db.as_ref())
                .await
                .map_err(|e| AppError::InternalServerError(e.to_string()))?
                .into_iter()
                .map(|r| (r.id, r.name))
                .collect::<std::collections::HashMap<_, _>>()
        } else {
            std::collections::HashMap::new()
        };

        let data: Vec<UserListItem> = users.into_iter().map(|user| {
            let role_name = user.role_id.and_then(|rid| roles.get(&rid).cloned()).unwrap_or_default();
            UserListItem {
                id: user.id.to_string(),
                role: role_name,
                fullname: format!("{} {}",
                    user.first_name.as_deref().unwrap_or(""),
                    user.last_name.as_deref().unwrap_or("")
                ).trim().to_string(),
                email: user.email,
                avatar: user.avatar_url,
                is_active: user.is_active,
                created_at: user.created_at.to_rfc3339(),
                updated_at: user.updated_at.to_rfc3339(),
            }
        }).collect();

        let total = paginator.num_items().await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        let meta = PaginatorResponseMeta::new(page, per_page, total as u32);
        Ok(PaginatorResponse { data, meta })
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

        let role_dto = build_role_dto(role);

        let mut dto = UsersDetailQueryDto::default();
        dto.id = user.id.to_string();
        dto.fullname = format!("{} {}",
            user.first_name.as_deref().unwrap_or(""),
            user.last_name.as_deref().unwrap_or("")
        ).trim().to_string();
        dto.legal_name = None;
        dto.email = user.email;
        dto.avatar = user.avatar_url;
        dto.is_active = user.is_active;
        dto.is_deleted = user.deleted_at.is_some();
        dto.profile_extension = user.metadata.and_then(|m| serde_json::from_value(m).ok());
        dto.password = user.password_hash;
        dto.role = role_dto;
        dto.created_at = user.created_at.to_rfc3339();
        dto.updated_at = user.updated_at.to_rfc3339();
        dto.mentor_id = None;

        Ok(user_detail_to_entity(dto.from_profile_extension()))
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

        let role_dto = build_role_dto(role);

        let mut dto = UsersDetailQueryDto::default();
        dto.id = user.id.to_string();
        dto.fullname = format!("{} {}",
            user.first_name.as_deref().unwrap_or(""),
            user.last_name.as_deref().unwrap_or("")
        ).trim().to_string();
        dto.legal_name = None;
        dto.email = user.email;
        dto.avatar = user.avatar_url;
        dto.is_active = user.is_active;
        dto.is_deleted = user.deleted_at.is_some();
        dto.profile_extension = user.metadata.and_then(|m| serde_json::from_value(m).ok());
        dto.password = user.password_hash;
        dto.role = role_dto;
        dto.created_at = user.created_at.to_rfc3339();
        dto.updated_at = user.updated_at.to_rfc3339();
        dto.mentor_id = None;

        Ok(user_detail_to_entity(dto.from_profile_extension()))
    }

    async fn create(&self, entity: UserEntity) -> Result<String, AppError> {
        // Check for existing user
        let existing = UsersEntity::find()
            .filter(UserColumn::Email.eq(entity.email.clone()))
            .one(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if existing.is_some() {
            return Err(AppError::ConflictError("User with this email already exists".into()));
        }

        let full_name = entity.fullname.clone();
        let (first_name, last_name) = full_name.split_once(' ').unwrap_or((&full_name, ""));

        let role_id = entity.role.id.parse::<Uuid>().ok()
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
                entity.profile_extension.map(|p| serde_json::to_value(p).unwrap_or_default())
            ),
            created_at: ActiveValue::Set(Utc::now()),
            updated_at: ActiveValue::Set(Utc::now()),
            deleted_at: ActiveValue::Set(None),
            role_id: ActiveValue::Set(role_id.filter(|id| !id.is_nil())),
        };

        UsersEntity::insert(active_model).exec(self.db.as_ref()).await
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
        let (first_name, last_name) = full_name.split_once(' ').unwrap_or((&full_name, ""));

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
                entity.profile_extension.map(|p| serde_json::to_value(p).unwrap_or_default())
            );
        }

        active_model.update(self.db.as_ref()).await
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

        active_model.update(self.db.as_ref()).await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok("Success delete user".into())
    }
}
