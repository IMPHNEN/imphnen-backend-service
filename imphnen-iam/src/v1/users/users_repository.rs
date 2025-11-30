use imphnen_entities::{
    UsersDetailQueryDto,
    seaorm::auth::users::{Entity as UsersEntity, ActiveModel as UserActiveModel, Column as UserColumn},
    seaorm::auth::roles::{Entity as RolesEntity},
    RolesDetailQueryDto,
    PermissionsQueryDto // Import this
};
use crate::{
    UsersSchema,
    AppState,
    MetaRequestDto,
    ResponseListSuccessDto,
    MetaResponseDto
};
use super::users_dto::UsersListItemDto;
use sea_orm::{
    EntityTrait,
    QueryFilter,
    PaginatorTrait,
    ActiveModelTrait,
    ActiveValue,
    DatabaseConnection,
    ColumnTrait,
};
use anyhow::{Result, bail, anyhow};
use uuid::Uuid;
use chrono::Utc;
use std::time::Instant;

pub struct UsersRepository<'a> {
	state: &'a AppState,
}

impl<'a> UsersRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	fn db(&self) -> &DatabaseConnection {
		&self.state.postgres_connection.conn
	}

    pub async fn query_user_list(
        &self,
        meta: MetaRequestDto,
    ) -> Result<ResponseListSuccessDto<Vec<UsersListItemDto>>> {
        let now = Instant::now();
        
        // Build the query
        let query = UsersEntity::find()
            .filter(UserColumn::DeletedAt.is_null())
            .filter(UserColumn::IsActive.eq(true));

        // Apply search if provided
        // if let Some(search) = &meta.search {
        //     query = query.filter(
        //         UserColumn::Email.contains(search)
        //             .or(UserColumn::Username.contains(search))
        //             .or(UserColumn::FirstName.contains(search))
        //             .or(UserColumn::LastName.contains(search))
        //     );
        // } // Re-add search capability once query building is fixed

        // Apply ordering
        // query = query.order_by(UserColumn::CreatedAt, sea_orm::Order::Desc); // Re-add ordering

        // Apply pagination
        let page = meta.page.unwrap_or(1);
        let per_page = meta.per_page.unwrap_or(10);
        let paginator = query.paginate(self.db(), per_page);
        let users = paginator.fetch_page(page - 1).await?;
        
        // Optimize: Load all roles in one query
        let role_ids: Vec<Uuid> = users.iter().filter_map(|u| u.role_id).collect();
        let roles = if !role_ids.is_empty() {
            RolesEntity::find()
                .filter(imphnen_entities::seaorm::auth::roles::Column::Id.is_in(role_ids))
                .all(self.db())
                .await?
                .into_iter()
                .map(|r| (r.id, r.name))
                .collect::<std::collections::HashMap<_, _>>()
        } else {
            std::collections::HashMap::new()
        };
        
        // Convert to DTOs
        let mut data: Vec<UsersListItemDto> = Vec::with_capacity(users.len());
        for user in users.into_iter() {
            let role_name = user.role_id.and_then(|rid| roles.get(&rid).cloned()).unwrap_or_default();
            data.push(UsersListItemDto {
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
            });
        }


        let total = paginator.num_items().await?;
        
        let meta_response = MetaResponseDto {
            page: Some(page),
            per_page: Some(per_page),
            total: Some(total),
        };

        let elapsed = now.elapsed();

        if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
            == "development"
        {
            println!("Query 'query_user_list' took: {elapsed:.2?}");
        }

        Ok(ResponseListSuccessDto {
            data,
            meta: Some(meta_response),
        })
    }


    pub async fn query_user_by_email(
        &self,
        email: String,
    ) -> Result<UsersDetailQueryDto> {
        let now = Instant::now();
        
        let user_and_role = UsersEntity::find()
            .filter(UserColumn::Email.eq(email))
            .filter(UserColumn::DeletedAt.is_null())
            .find_also_related(RolesEntity)
            .one(self.db())
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        let (user, role) = user_and_role;
        let role_dto = role.map_or_else(RolesDetailQueryDto::default, |r| RolesDetailQueryDto {
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
        });

        let elapsed = now.elapsed();

        if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
            == "development"
        {
            println!("Query 'query_user_by_email' took: {elapsed:.2?}");
        }

        // Convert UserModel to UsersDetailQueryDto
        let mut dto = UsersDetailQueryDto::default();
        dto.id = user.id.to_string();
        dto.fullname = format!("{} {}", user.first_name.as_deref().unwrap_or(""), user.last_name.as_deref().unwrap_or("")).trim().to_string();
        dto.legal_name = None;
        dto.email = user.email;
        dto.avatar = user.avatar_url;
        dto.is_active = user.is_active;
        dto.is_deleted = user.deleted_at.is_some();
        dto.profile_extension = user.metadata.and_then(|m| serde_json::from_value(m).ok()); // Extract from metadata
        dto.password = String::new(); // Don't expose password
        dto.role = role_dto; // Use actual role DTO
        dto.created_at = user.created_at.to_rfc3339();
        dto.updated_at = user.updated_at.to_rfc3339();
        dto.mentor_id = None; // Not in model
        Ok(dto.from_profile_extension())
    }


    pub async fn query_user_by_id(&self, id: &str) -> Result<UsersDetailQueryDto> {
        let now = Instant::now();
        
        let user_id = Uuid::parse_str(id)?;
        let user_and_role = UsersEntity::find_by_id(user_id)
            .filter(UserColumn::DeletedAt.is_null())
            .find_also_related(RolesEntity)
            .one(self.db())
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found in database"))?;

        let (user, role) = user_and_role;
        let role_dto = role.map_or_else(RolesDetailQueryDto::default, |r| RolesDetailQueryDto {
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
        });

        let elapsed = now.elapsed();

        if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
            == "development"
        {
            println!("Query 'query_user_by_id' took: {elapsed:.2?}");
        }

        // Convert UserModel to UsersDetailQueryDto
        let mut dto = UsersDetailQueryDto::default();
        dto.id = user.id.to_string();
        dto.fullname = format!("{} {}", user.first_name.as_deref().unwrap_or(""), user.last_name.as_deref().unwrap_or("")).trim().to_string();
        dto.legal_name = None;
        dto.email = user.email;
        dto.avatar = user.avatar_url;
        dto.is_active = user.is_active;
        dto.is_deleted = user.deleted_at.is_some();
        dto.profile_extension = user.metadata.and_then(|m| serde_json::from_value(m).ok()); // Extract from metadata
        dto.password = String::new(); // Don't expose password
        dto.role = role_dto; // Use actual role DTO
        dto.created_at = user.created_at.to_rfc3339();
        dto.updated_at = user.updated_at.to_rfc3339();
        dto.mentor_id = None; // Not in model
        Ok(dto.from_profile_extension())
    }


    pub async fn query_create_user(&self, data: UsersSchema) -> Result<String> {
        let now = Instant::now();
        
        // Check if user already exists
        let existing_user = UsersEntity::find()
            .filter(UserColumn::Email.eq(data.email.clone()))
            .one(self.db())
            .await?;
        
        if existing_user.is_some() {
            bail!("User with this email already exists");
        }

        let email = data.email.ok_or_else(|| anyhow!("Email is required"))?;
        let password = data.password.ok_or_else(|| anyhow!("Password is required"))?;
        let full_name = data.fullname.unwrap_or_default();
        let (first_name, last_name) = full_name.split_once(' ').unwrap_or((&full_name, ""));

        let user_active_model = UserActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            email: ActiveValue::Set(email.clone()),
            password_hash: ActiveValue::Set(password),
            username: ActiveValue::Set(email.clone()), // Use email as username for now
            first_name: ActiveValue::Set(Some(first_name.to_string())),
            last_name: ActiveValue::Set(Some(last_name.to_string())),
            avatar_url: ActiveValue::Set(data.avatar),
            is_verified: ActiveValue::Set(false),
            is_active: ActiveValue::Set(data.is_active),
            metadata: ActiveValue::Set(data.profile_extension.map(|p| serde_json::to_value(p).unwrap_or_default())), // Save profile_extension to metadata
            created_at: ActiveValue::Set(Utc::now()),
            updated_at: ActiveValue::Set(Utc::now()),
            deleted_at: ActiveValue::Set(None),
            role_id: ActiveValue::Set(data.role_id), // Add this line
        };

        let _result = UsersEntity::insert(user_active_model).exec(self.db()).await?;
        
        let elapsed = now.elapsed();

        if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
            == "development"
        {
            println!("Query 'query_create_user' took: {elapsed:.2?}");
        }

        Ok("Successfully created user".into())
    }


    pub async fn query_update_user(&self, data: UsersSchema) -> Result<String> {
        let now = Instant::now();
        
        let user_id = Uuid::parse_str(&data.id)?;
        let existing = self.query_user_by_id(&data.id).await?;
        if existing.is_deleted {
            bail!("User already deleted");
        }

        let mut user_active_model: UserActiveModel = UsersEntity::find_by_id(user_id)
            .one(self.db())
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?
            .into();

        let email = data.email.ok_or_else(|| anyhow!("Email is required"))?;
        let full_name = data.fullname.unwrap_or_default();
        let (first_name, last_name) = full_name.split_once(' ').unwrap_or((&full_name, ""));

        // Update fields
        user_active_model.email = ActiveValue::Set(email);
        user_active_model.first_name = ActiveValue::Set(Some(first_name.to_string()));
        user_active_model.last_name = ActiveValue::Set(Some(last_name.to_string()));
        user_active_model.avatar_url = ActiveValue::Set(data.avatar);
        user_active_model.is_active = ActiveValue::Set(data.is_active);
        user_active_model.updated_at = ActiveValue::Set(chrono::Utc::now());
        if data.role_id.is_some() { // Conditionally update role_id
            user_active_model.role_id = ActiveValue::Set(data.role_id);
        }
        if data.profile_extension.is_some() { // Conditionally update profile_extension
            user_active_model.metadata = ActiveValue::Set(data.profile_extension.map(|p| serde_json::to_value(p).unwrap_or_default()));
        }

        let _result = user_active_model.update(self.db()).await?;
        
        let elapsed = now.elapsed();

        if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
            == "development"
        {
            println!("Query 'query_update_user' took: {elapsed:.2?}");
        }

        Ok("Success update user".into())
    }


    pub async fn query_delete_user(&self, id: String) -> Result<String> {
        let now = Instant::now();
        
        let user_id = Uuid::parse_str(&id)?;
        let user = self.query_user_by_id(&id).await?;
        if user.is_deleted {
            bail!("User not found");
        }

        let mut user_active_model: UserActiveModel = UsersEntity::find_by_id(user_id)
            .one(self.db())
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?
            .into();

        // Soft delete
        user_active_model.deleted_at = ActiveValue::Set(Some(chrono::Utc::now()));
        user_active_model.updated_at = ActiveValue::Set(chrono::Utc::now());

        let _result = user_active_model.update(self.db()).await?;
            
        let elapsed = now.elapsed();

        if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
            == "development"
        {
            println!("Query 'query_delete_user' took: {elapsed:.2?}");
        }

        Ok("Success delete user".into())
    }
}
