use crate::gacha_items::domain::{
	gacha_item::GachaItemEntity, repository::GachaItemRepository,
};
use async_trait::async_trait;
use imphnen_entities::seaorm::gacha::gacha_items::{
	ActiveModel as GachaItemsActiveModel, Column as GachaItemsColumn,
	Entity as GachaItemsEntity, Model as GachaItemsModel,
};
use imphnen_utils::AppError;
use paginator_rs::{PaginationParams, SortDirection};
use paginator_utils::{PaginatorResponse, PaginatorResponseMeta};
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, Order, PaginatorTrait, QueryOrder};
use std::sync::Arc;
use uuid::Uuid;

fn to_entity(model: GachaItemsModel) -> GachaItemEntity {
	GachaItemEntity {
		id: model.id,
		item_code: model.item_code,
		name: model.name,
		description: model.description,
		rarity: model.rarity,
		type_: model.type_,
		category: model.category,
		value: model.value,
		weight: model.weight,
		stock: model.stock,
		is_limited: model.is_limited,
		metadata: model.metadata,
		is_deleted: model.deleted_at.is_some(),
		created_at: model.created_at,
		updated_at: model.updated_at,
		deleted_at: model.deleted_at,
	}
}

pub struct PostgresGachaItemRepository {
	db: Arc<DatabaseConnection>,
}

impl PostgresGachaItemRepository {
	pub fn new(db: DatabaseConnection) -> Self {
		Self { db: Arc::new(db) }
	}
}

#[async_trait]
impl GachaItemRepository for PostgresGachaItemRepository {
	async fn find_all(
		&self,
		params: PaginationParams,
	) -> Result<PaginatorResponse<GachaItemEntity>, AppError> {
		let page = params.page.max(1);
		let per_page = params.per_page.clamp(1, 100);

		let mut query =
			GachaItemsEntity::find().filter(GachaItemsColumn::DeletedAt.is_null());

		if let Some(ref search) = params.search {
			query = query.filter(GachaItemsColumn::Name.contains(&search.query));
		}

		query = match params.sort_by.as_deref() {
			Some("name") => match params.sort_direction {
				Some(SortDirection::Desc) => {
					query.order_by(GachaItemsColumn::Name, Order::Desc)
				}
				_ => query.order_by(GachaItemsColumn::Name, Order::Asc),
			},
			_ => match params.sort_direction {
				Some(SortDirection::Asc) => {
					query.order_by(GachaItemsColumn::CreatedAt, Order::Asc)
				}
				_ => query.order_by(GachaItemsColumn::CreatedAt, Order::Desc),
			},
		};

		let paginator = query.paginate(self.db.as_ref(), per_page as u64);
		let total = paginator
			.num_items()
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		let items = paginator
			.fetch_page((page - 1) as u64)
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		let data = items.into_iter().map(to_entity).collect();
		let meta = PaginatorResponseMeta::new(page, per_page, total as u32);
		Ok(PaginatorResponse { data, meta })
	}

	async fn find_by_id(&self, id: Uuid) -> Result<GachaItemEntity, AppError> {
		let item = GachaItemsEntity::find_by_id(id)
			.filter(GachaItemsColumn::DeletedAt.is_null())
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?
			.ok_or_else(|| AppError::NotFoundError("Gacha item not found".to_string()))?;

		Ok(to_entity(item))
	}

	async fn create(&self, entity: GachaItemEntity) -> Result<(), AppError> {
		let active_model = GachaItemsActiveModel {
			id: ActiveValue::Set(entity.id),
			item_code: ActiveValue::Set(entity.item_code),
			name: ActiveValue::Set(entity.name),
			description: ActiveValue::Set(entity.description),
			rarity: ActiveValue::Set(entity.rarity),
			type_: ActiveValue::Set(entity.type_),
			category: ActiveValue::Set(entity.category),
			value: ActiveValue::Set(entity.value),
			weight: ActiveValue::Set(entity.weight),
			stock: ActiveValue::Set(entity.stock),
			is_limited: ActiveValue::Set(entity.is_limited),
			metadata: ActiveValue::Set(entity.metadata),
			created_at: ActiveValue::Set(chrono::Utc::now()),
			updated_at: ActiveValue::Set(chrono::Utc::now()),
			deleted_at: ActiveValue::Set(None),
		};

		GachaItemsEntity::insert(active_model)
			.exec(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		Ok(())
	}

	async fn update(&self, entity: GachaItemEntity) -> Result<(), AppError> {
		let mut active_model: GachaItemsActiveModel =
			GachaItemsEntity::find_by_id(entity.id)
				.one(self.db.as_ref())
				.await
				.map_err(|e| AppError::InternalServerError(e.to_string()))?
				.ok_or_else(|| AppError::NotFoundError("Gacha item not found".to_string()))?
				.into();

		active_model.item_code = ActiveValue::Set(entity.item_code);
		active_model.name = ActiveValue::Set(entity.name);
		active_model.description = ActiveValue::Set(entity.description);
		active_model.rarity = ActiveValue::Set(entity.rarity);
		active_model.type_ = ActiveValue::Set(entity.type_);
		active_model.category = ActiveValue::Set(entity.category);
		active_model.value = ActiveValue::Set(entity.value);
		active_model.weight = ActiveValue::Set(entity.weight);
		active_model.stock = ActiveValue::Set(entity.stock);
		active_model.is_limited = ActiveValue::Set(entity.is_limited);
		active_model.metadata = ActiveValue::Set(entity.metadata);
		active_model.updated_at = ActiveValue::Set(chrono::Utc::now());

		active_model
			.update(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		Ok(())
	}

	async fn delete(&self, id: Uuid) -> Result<(), AppError> {
		let mut active_model: GachaItemsActiveModel = GachaItemsEntity::find_by_id(id)
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?
			.ok_or_else(|| AppError::NotFoundError("Gacha item not found".to_string()))?
			.into();

		active_model.deleted_at = ActiveValue::Set(Some(chrono::Utc::now()));
		active_model.updated_at = ActiveValue::Set(chrono::Utc::now());

		active_model
			.update(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		Ok(())
	}
}
