use super::{
	RolesDetailItemDto, RolesListItemDto, RolesRequestCreateDto,
	RolesRequestUpdateDto, RolesSchema,
};
use crate::{
	AppState, MetaRequestDto, ResponseListSuccessDto,
};
use anyhow::Result;
use imphnen_entities::seaorm::auth::roles::{Entity as RolesEntity, Column as RolesColumn};
use sea_orm::{EntityTrait, QueryFilter, QueryOrder, PaginatorTrait, ActiveModelTrait, ActiveValue, DatabaseConnection, ColumnTrait, QuerySelect, Order};
use serde_json;
use std::time::Instant;
use uuid::Uuid;
use tracing::instrument;

pub struct RolesRepository<'a> {
	state: &'a AppState,
}

impl<'a> RolesRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	fn db(&self) -> &DatabaseConnection {
		&self.state.postgres_connection.conn
	}

	#[instrument(skip(self, meta), err)]
	pub async fn query_role_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<RolesListItemDto>>> {
		let now = Instant::now();
		
		// Build SeaORM query
		let mut query = RolesEntity::find()
			.filter(RolesColumn::DeletedAt.is_null());
		
		// Apply search filter
		if let Some(search) = &meta.search {
			query = query.filter(RolesColumn::Name.contains(search));
		}
		
		// Apply sorting
		let sort_column = match meta.sort_by.as_deref() {
			Some("name") => RolesColumn::Name,
			Some("created_at") => RolesColumn::CreatedAt,
			_ => RolesColumn::CreatedAt,
		};
		
		query = match meta.order.as_deref() {
			Some("desc") => query.order_by(sort_column, Order::Desc),
			_ => query.order_by(sort_column, Order::Asc),
		};
		
		// Get total count
		let total_count = query.clone().count(self.db()).await?;
		
		// Apply pagination
		let page = meta.page.unwrap_or(1);
		let per_page = meta.per_page.unwrap_or(10);
		let offset = (page - 1) * per_page;
		
		let roles = query
			.offset(offset)
			.limit(per_page)
			.all(self.db())
			.await?;
		
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_role_list' took: {elapsed:.2?}");
		}
		
		let data = roles
			.into_iter()
			.map(|role| {
				let schema = RolesSchema::from(&role);
				schema.list()
			})
			.collect();
		
		Ok(ResponseListSuccessDto {
			data,
			meta: Some(imphnen_entities::MetaResponseDto {
				total: Some(total_count),
				page: Some(page),
				per_page: Some(per_page),
			}),
		})
	}

	#[instrument(skip(self, name), err)]
	pub async fn query_role_by_name(
		&self,
		name: String,
	) -> Result<RolesDetailItemDto> {
		let now = Instant::now();
		
		let role = RolesEntity::find()
			.filter(RolesColumn::Name.eq(name))
			.filter(RolesColumn::DeletedAt.is_null())
			.one(self.db())
			.await?
			.ok_or_else(|| anyhow::anyhow!("Role not found"))?;

		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_role_by_name' took: {elapsed:.2?}");
		}
		Ok(RolesDetailItemDto::from(&role))
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_role_by_id(&self, id: String) -> Result<RolesDetailItemDto> {
		let now = Instant::now();
		
		let role_id = Uuid::parse_str(&id)
			.map_err(|_| anyhow::anyhow!("Invalid role ID"))?;
		
		let role = RolesEntity::find_by_id(role_id)
			.filter(RolesColumn::DeletedAt.is_null())
			.one(self.db())
			.await?
			.ok_or_else(|| anyhow::anyhow!("Role not found"))?;

		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_role_by_id' took: {elapsed:.2?}");
		}
		Ok(RolesDetailItemDto::from(&role))
	}

	#[instrument(skip(self, payload), err)]
	pub async fn query_create_role(
		&self,
		payload: RolesRequestCreateDto,
	) -> Result<RolesDetailItemDto> {
		let now = Instant::now();
		
		let role_id = Uuid::new_v4();
		let permissions_json = serde_json::to_value(&payload.permissions)
			.map_err(|e| anyhow::anyhow!("Failed to serialize permissions: {}", e))?;
		
		let active_model = imphnen_entities::seaorm::auth::roles::ActiveModel {
			id: ActiveValue::Set(role_id),
			name: ActiveValue::Set(payload.name),
			description: ActiveValue::Set("".to_string()), // Default description
			is_system_role: ActiveValue::Set(false),
			is_default: ActiveValue::Set(false),
			permissions: ActiveValue::Set(Some(permissions_json)),
			created_at: ActiveValue::Set(chrono::Utc::now()),
			updated_at: ActiveValue::Set(chrono::Utc::now()),
			deleted_at: ActiveValue::NotSet,
		};
		
		let created_role = active_model.insert(self.db()).await?;
		
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_create_role' took: {elapsed:.2?}");
		}
		
		// Return the created role
		Ok(RolesDetailItemDto::from(&created_role))
	}

	#[instrument(skip(self, id, data), err)]
	pub async fn query_update_role(
		&self,
		id: String,
		data: RolesRequestUpdateDto,
	) -> Result<String> {
		let now = Instant::now();
		
		let role_id = Uuid::parse_str(&id)
			.map_err(|_| anyhow::anyhow!("Invalid role ID"))?;
		
		let mut active_model = imphnen_entities::seaorm::auth::roles::ActiveModel {
			id: ActiveValue::Unchanged(role_id),
			..Default::default()
		};
		
		if let Some(name) = data.name {
			active_model.name = ActiveValue::Set(name);
		}
		
		if let Some(permissions) = data.permissions {
			let permissions_json = serde_json::to_value(&permissions)
				.map_err(|e| anyhow::anyhow!("Failed to serialize permissions: {}", e))?;
			active_model.permissions = ActiveValue::Set(Some(permissions_json));
		}
		
		active_model.updated_at = ActiveValue::Set(chrono::Utc::now());
		
		let _updated_role = active_model.update(self.db()).await?;
		
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_update_role' took: {elapsed:.2?}");
		}
		
		Ok("Success update role".into())
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_delete_role(&self, id: String) -> Result<String> {
		let now = Instant::now();
		
		let role_id = Uuid::parse_str(&id)
			.map_err(|_| anyhow::anyhow!("Invalid role ID"))?;
		
		let active_model = imphnen_entities::seaorm::auth::roles::ActiveModel {
			id: ActiveValue::Unchanged(role_id),
			deleted_at: ActiveValue::Set(Some(chrono::Utc::now())),
			..Default::default()
		};
		
		let _updated_role = active_model.update(self.db()).await?;
		
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_delete_role' took: {elapsed:.2?}");
		}
		
		Ok("Success delete role".into())
	}
}
