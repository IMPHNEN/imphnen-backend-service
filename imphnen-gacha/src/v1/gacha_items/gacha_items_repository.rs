use crate::v1::gacha_items::gacha_items_schema::GachaItemSchema;
use crate::v1::gacha_items::GachaItemDto;
use crate::{AppState, MetaRequestDto, ResponseListSuccessDto};
use anyhow::{Result, bail};
// QueryListBuilder is not available in imphnen-iam, need to implement locally or use alternative
use std::time::Instant;
use tracing::instrument;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, QueryOrder, PaginatorTrait, ActiveModelTrait, ActiveValue, QuerySelect};
use imphnen_entities::seaorm::gacha::gacha_items::{Entity as GachaItemEntity, Column as GachaItemColumn, ActiveModel as GachaItemActiveModel};
use uuid::Uuid;

pub struct GachaItemRepository<'a> {
	state: &'a AppState,
}

impl<'a> GachaItemRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	#[instrument(skip(self, meta), err)]
	pub async fn query_gacha_item_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<GachaItemDto>>> {
		let now = Instant::now();
		let db = &self.state.postgres_connection.conn;
		
		let query = GachaItemEntity::find()
			.filter(GachaItemColumn::DeletedAt.is_null());
		
		// Apply search if provided
		let query = if let Some(search) = &meta.search {
			query.filter(GachaItemColumn::Name.contains(search))
		} else {
			query
		};
		
		// Apply sorting
		let query = if let Some(sort_by) = &meta.sort_by {
			match sort_by.as_str() {
				"name" => {
					if meta.order.as_deref() == Some("desc") {
						query.order_by_desc(GachaItemColumn::Name)
					} else {
						query.order_by_asc(GachaItemColumn::Name)
					}
				}
				"created_at" => {
					if meta.order.as_deref() == Some("desc") {
						query.order_by_desc(GachaItemColumn::CreatedAt)
					} else {
						query.order_by_asc(GachaItemColumn::CreatedAt)
					}
				}
				_ => query.order_by_desc(GachaItemColumn::CreatedAt),
			}
		} else {
			query.order_by_desc(GachaItemColumn::CreatedAt)
		};
		
		// Get total count
		let total_count = query.clone().count(db).await?;
		
		// Apply pagination
		let page = meta.page.unwrap_or(1);
		let limit = meta.per_page.unwrap_or(10);
		let offset = (page - 1) * limit;
		
		let items = query
			.offset(offset)
			.limit(limit)
			.all(db)
			.await?;
		
		let data: Vec<GachaItemDto> = items
			.into_iter()
			.map(|item| GachaItemDto {
				id: item.id.to_string(),
				name: item.name,
				is_deleted: item.deleted_at.is_some(),
				created_at: Some(item.created_at.to_string()),
				updated_at: Some(item.updated_at.to_string()),
			})
			.collect();
		
		let _total_pages = (total_count as f64 / limit as f64).ceil() as u32;
		
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_gacha_item_list' took: {elapsed:.2?}");
		}
		
		Ok(ResponseListSuccessDto {
			data,
			meta: Some(imphnen_libs::MetaResponseDto {
				page: Some(page),
				per_page: Some(limit),
				total: Some(total_count),
			}),
		})
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_gacha_item_by_id(&self, id: String) -> Result<GachaItemSchema> {
		let now = Instant::now();
		let db = &self.state.postgres_connection.conn;
		
		let uuid_id = Uuid::parse_str(&id)?;
		
		let item = GachaItemEntity::find_by_id(uuid_id)
			.filter(GachaItemColumn::DeletedAt.is_null())
			.one(db)
			.await?;
		
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_gacha_item_by_id' took: {elapsed:.2?}");
		}
		
		match item {
			Some(item) => Ok(GachaItemSchema {
				id: item.id.to_string(),
				item_code: item.item_code,
				name: item.name,
				description: item.description,
				rarity: item.rarity,
				type_: item.type_,
				category: item.category,
				value: item.value,
				weight: item.weight,
				stock: item.stock,
				is_limited: item.is_limited,
				metadata: item.metadata,
				image_url: "".to_string(), // Not present in DB model
				is_deleted: item.deleted_at.is_some(),
				created_at: Some(item.created_at.to_string()),
				updated_at: Some(item.updated_at.to_string()),
			}),
			None => bail!("Gacha Item not found"),
		}
	}

	#[instrument(skip(self, data), err)]
	pub async fn query_create_gacha_item(
		&self,
		data: GachaItemSchema,
	) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.postgres_connection.conn;
		
		let active_model = GachaItemActiveModel {
			id: ActiveValue::Set(Uuid::new_v4()),
			item_code: ActiveValue::Set(data.item_code),
			name: ActiveValue::Set(data.name),
			description: ActiveValue::Set(data.description),
			rarity: ActiveValue::Set(data.rarity),
			type_: ActiveValue::Set(data.type_),
			category: ActiveValue::Set(data.category),
			value: ActiveValue::Set(data.value),
			weight: ActiveValue::Set(data.weight),
			stock: ActiveValue::Set(data.stock),
			is_limited: ActiveValue::Set(data.is_limited),
			metadata: ActiveValue::Set(data.metadata),
			created_at: ActiveValue::Set(chrono::Utc::now()),
			updated_at: ActiveValue::Set(chrono::Utc::now()),
			deleted_at: ActiveValue::NotSet,
		};
		
		let result = active_model.insert(db).await?;
		
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_create_gacha_item' took: {elapsed:.2?}");
		}
		
		Ok(result.id.to_string())
	}

	#[instrument(skip(self, data), err)]
	pub async fn query_update_gacha_item(
		&self,
		data: GachaItemSchema,
	) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.postgres_connection.conn;
		
		let uuid_id = Uuid::parse_str(&data.id)?;
		
		let mut active_model: GachaItemActiveModel = GachaItemEntity::find_by_id(uuid_id)
			.one(db)
			.await?
			.ok_or_else(|| anyhow::anyhow!("Gacha Item not found"))?
			.into();
		
		active_model.item_code = ActiveValue::Set(data.item_code);
		active_model.name = ActiveValue::Set(data.name);
		active_model.description = ActiveValue::Set(data.description);
		active_model.rarity = ActiveValue::Set(data.rarity);
		active_model.type_ = ActiveValue::Set(data.type_);
		active_model.category = ActiveValue::Set(data.category);
		active_model.value = ActiveValue::Set(data.value);
		active_model.weight = ActiveValue::Set(data.weight);
		active_model.stock = ActiveValue::Set(data.stock);
		active_model.is_limited = ActiveValue::Set(data.is_limited);
		active_model.metadata = ActiveValue::Set(data.metadata);
		active_model.updated_at = ActiveValue::Set(chrono::Utc::now());
		
		let _result = active_model.update(db).await?;
		
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_update_gacha_item' took: {elapsed:.2?}");
		}
		
		Ok("Success update Gacha Item".into())
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_delete_gacha_item(&self, id: String) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.postgres_connection.conn;
		
		let uuid_id = Uuid::parse_str(&id)?;
		
		let mut active_model: GachaItemActiveModel = GachaItemEntity::find_by_id(uuid_id)
			.one(db)
			.await?
			.ok_or_else(|| anyhow::anyhow!("Gacha Item not found"))?
			.into();
		
		if active_model.deleted_at.is_set() {
			bail!("Gacha Item already deleted");
		}
		
		active_model.deleted_at = ActiveValue::Set(Some(chrono::Utc::now()));
		active_model.updated_at = ActiveValue::Set(chrono::Utc::now());
		
		active_model.update(db).await?;
		
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_delete_gacha_item' took: {elapsed:.2?}");
		}
		
		Ok("Success soft delete Gacha Item".into())
	}
}
