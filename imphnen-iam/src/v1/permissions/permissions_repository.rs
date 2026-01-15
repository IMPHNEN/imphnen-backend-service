use imphnen_entities::{PermissionsItemDto, MetaRequestDto, ResponseListSuccessDto};
use super::PermissionsSchema;
use crate::{AppState};
use imphnen_libs::AppStatePostgresExt;
use imphnen_entities::seaorm::auth::permissions::Entity as PermissionsEntity;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, QueryOrder, PaginatorTrait, QuerySelect, ActiveModelTrait, ActiveValue};
use anyhow::{Result, bail};
use std::time::Instant;
use tracing::instrument;
use tracing::info;
use uuid::Uuid;

pub struct PermissionsRepository<'a> {
	state: &'a AppState,
}

impl<'a> PermissionsRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	#[instrument(skip(self, meta), err)]
	pub async fn query_permission_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<PermissionsItemDto>>> {
		let now = Instant::now();
		info!("Executing SeaORM query for Permissions list with meta: {:?}", meta);

		let db = self.state.postgres_db();

		// Build base query
		let mut query = PermissionsEntity::find()
			.filter(imphnen_entities::seaorm::auth::permissions::Column::IsDeleted.eq(false));

		// Apply search if provided
		if let Some(search) = &meta.search {
			query = query.filter(imphnen_entities::seaorm::auth::permissions::Column::Name.contains(search));
		}

		// Apply ordering
		let order_column = match meta.sort_by.as_deref() {
		    Some("name") => imphnen_entities::seaorm::auth::permissions::Column::Name,
		    Some("created_at") => imphnen_entities::seaorm::auth::permissions::Column::CreatedAt,
		    _ => imphnen_entities::seaorm::auth::permissions::Column::CreatedAt,
		};

		query = match meta.order.as_deref() {
		    Some("desc") => query.order_by_desc(order_column),
		    _ => query.order_by_asc(order_column),
		};

		// Get total count for pagination
		let total_count = query.clone().count(db).await?;

		// Apply pagination
		let page = meta.page.unwrap_or(1);
		let per_page = meta.per_page.unwrap_or(10);
		let offset = (page - 1) * per_page;

		let permissions = query
			.offset(offset)
			.limit(per_page)
			.all(db)
			.await?;

		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()) == "development" {
			println!("Query 'query_permission_list' took: {elapsed:.2?}");
		}

		let transformed_data = permissions
			.into_iter()
			.map(|permission| PermissionsItemDto {
				id: permission.id.to_string(),
				name: permission.name,
				created_at: Some(permission.created_at.to_rfc3339()),
				updated_at: Some(permission.updated_at.to_rfc3339()),
			})
			.collect();

		Ok(ResponseListSuccessDto {
			data: transformed_data,
			meta: Some(imphnen_entities::MetaResponseDto {
				page: Some(page),
				per_page: Some(per_page),
				total: Some(total_count),
			}),
		})
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_permission_by_id(
		&self,
		id: String,
	) -> Result<PermissionsSchema> {
		let now = Instant::now();
		let db = self.state.postgres_db();
		info!(id = %id, "Executing SeaORM select for Permissions");

		let permission_id = Uuid::parse_str(&id)?;

		let permission = PermissionsEntity::find_by_id(permission_id)
			.filter(imphnen_entities::seaorm::auth::permissions::Column::IsDeleted.eq(false))
			.one(db)
			.await?;

		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()) == "development" {
			println!("Query 'query_permission_by_id' took: {elapsed:.2?}");
		}

		match permission {
			Some(permission) => Ok(PermissionsSchema {
				id: permission.id,
				name: permission.name,
				is_deleted: permission.is_deleted,
				created_at: Some(permission.created_at.to_rfc3339()),
				updated_at: Some(permission.updated_at.to_rfc3339()),
			}),
			None => bail!("Permission not found"),
		}
	}

	#[instrument(skip(self, id), err)]
	pub async fn transformed_query_permission_by_id(
		&self,
		id: String,
	) -> Result<PermissionsItemDto> {
		let now = Instant::now();
		info!(id = %id, "Executing transformed_query_permission_by_id (delegates to query_permission_by_id)");
		let raw_result = self.query_permission_by_id(id.clone()).await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()) == "development" {
			println!("Query 'transformed_query_permission_by_id' took: {elapsed:.2?}");
		}
		let transformed_data = PermissionsItemDto {
			id: raw_result.id.to_string(),
			name: raw_result.name,
			created_at: raw_result.created_at,
			updated_at: raw_result.updated_at,
		};
		Ok(transformed_data)
	}

	#[instrument(skip(self, name), err)]
	pub async fn query_permission_by_name(
		&self,
		name: String,
	) -> Result<PermissionsSchema> {
		let now = Instant::now();
		let db = self.state.postgres_db();
		info!(name = %name, "Executing SeaORM query for permission by name");

		let permission = PermissionsEntity::find_by_name(&name)
			.filter(imphnen_entities::seaorm::auth::permissions::Column::IsDeleted.eq(false))
			.one(db)
			.await?;

		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()) == "development" {
			println!("Query 'query_permission_by_name' took: {elapsed:.2?}");
		}

		match permission {
			Some(permission) => Ok(PermissionsSchema {
				id: permission.id,
				name: permission.name,
				is_deleted: permission.is_deleted,
				created_at: Some(permission.created_at.to_rfc3339()),
				updated_at: Some(permission.updated_at.to_rfc3339()),
			}),
			None => bail!("Permission not found"),
		}
	}

	#[instrument(skip(self, data), err)]
	pub async fn query_create_permission(
		&self,
		data: PermissionsSchema,
	) -> Result<String> {
		let now = Instant::now();
		let db = self.state.postgres_db();
		info!("Executing SeaORM create for Permissions with data: {:?}", data);

		let active_model = imphnen_entities::seaorm::auth::permissions::ActiveModel {
			id: ActiveValue::Set(data.id),
			name: ActiveValue::Set(data.name),
			is_deleted: ActiveValue::Set(data.is_deleted),
			created_at: ActiveValue::Set(chrono::Utc::now()),
			updated_at: ActiveValue::Set(chrono::Utc::now()),
			deleted_at: ActiveValue::NotSet,
		};

		let result = PermissionsEntity::insert(active_model).exec(db).await?;

		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()) == "development" {
			println!("Query 'query_create_permission' took: {elapsed:.2?}");
		}

		Ok(format!("Success create permission with id: {}", result.last_insert_id))
	}

	#[instrument(skip(self, data), err)]
	pub async fn query_update_permission(
		&self,
		data: PermissionsSchema,
	) -> Result<String> {
		let now = Instant::now();
		let db = self.state.postgres_db();

		// Check if permission exists and is not deleted
		let existing = self.query_permission_by_id(data.id.to_string()).await?;
		if existing.is_deleted {
			bail!("Permission already deleted");
		}

		info!(id = %data.id, "Executing SeaORM update for Permissions");

		let active_model = imphnen_entities::seaorm::auth::permissions::ActiveModel {
			id: ActiveValue::Set(data.id),
			name: ActiveValue::Set(data.name),
			is_deleted: ActiveValue::Set(data.is_deleted),
			created_at: ActiveValue::Unchanged(existing.created_at.map(|s| {
			    chrono::DateTime::parse_from_rfc3339(&s)
			        .map_err(|e| anyhow::anyhow!("Failed to parse created_at: {}", e))
			        .unwrap()
			        .with_timezone(&chrono::Utc)
			})
			.unwrap_or(chrono::Utc::now())),
			updated_at: ActiveValue::Set(chrono::Utc::now()),
			deleted_at: ActiveValue::NotSet,
		};

		let result = active_model.update(db).await?;

		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()) == "development" {
			println!("Query 'query_update_permission' took: {elapsed:.2?}");
		}

		Ok(format!("Success update permission with id: {}", result.id))
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_delete_permission(&self, id: String) -> Result<String> {
		let now = Instant::now();
		let db = self.state.postgres_db();

		let permission_id = Uuid::parse_str(&id)?;

		// Check if permission exists and is not deleted
		let permission = PermissionsEntity::find_by_id(permission_id)
			.filter(imphnen_entities::seaorm::auth::permissions::Column::IsDeleted.eq(false))
			.one(db)
			.await?;

		let _permission = match permission {
		    Some(p) => p,
		    None => bail!("Permission not found"),
		};

		info!(id = %id, "Executing SeaORM soft delete for Permissions");

		let active_model = imphnen_entities::seaorm::auth::permissions::ActiveModel {
			id: ActiveValue::Set(permission_id),
			is_deleted: ActiveValue::Set(true),
			deleted_at: ActiveValue::Set(Some(chrono::Utc::now())),
			..Default::default()
		};

		let result = active_model.update(db).await?;

		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()) == "development" {
			println!("Query 'query_delete_permission' took: {elapsed:.2?}");
		}

		Ok(format!("Success delete permission with id: {}", result.id))
	}
}
