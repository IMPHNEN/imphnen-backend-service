use crate::events::domain::{event::EventEntity, repository::EventRepository};
use async_trait::async_trait;
use imphnen_entities::seaorm::common::events::{
	ActiveModel as EventsActiveModel, Column as EventsColumn, Entity as EventsEntity,
	Model as EventsModel,
};
use imphnen_utils::AppError;
use paginator_rs::{PaginationParams, SortDirection};
use paginator_utils::{PaginatorResponse, PaginatorResponseMeta};
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, Order, PaginatorTrait, QueryOrder};
use std::sync::Arc;
use uuid::Uuid;

fn to_entity(model: EventsModel) -> EventEntity {
	EventEntity {
		id: model.id,
		name: model.name,
		description: model.description,
		detail_link: model.detail_link,
		price: model.price,
		is_online: model.is_online,
		is_deleted: model.is_deleted,
		location: model.location,
		start_date: model.start_date,
		end_date: model.end_date,
		created_at: model.created_at,
		updated_at: model.updated_at,
	}
}

pub struct PostgresEventRepository {
	db: Arc<DatabaseConnection>,
}

impl PostgresEventRepository {
	pub fn new(db: DatabaseConnection) -> Self {
		Self { db: Arc::new(db) }
	}
}

#[async_trait]
impl EventRepository for PostgresEventRepository {
	async fn find_all(
		&self,
		params: PaginationParams,
	) -> Result<PaginatorResponse<EventEntity>, AppError> {
		let page = params.page.max(1);
		let per_page = params.per_page.clamp(1, 100);

		let mut query = EventsEntity::find().filter(EventsColumn::IsDeleted.eq(false));

		if let Some(ref search) = params.search {
			query = query.filter(EventsColumn::Name.contains(&search.query));
		}

		query = match params.sort_by.as_deref() {
			Some("name") => match params.sort_direction {
				Some(SortDirection::Desc) => query.order_by(EventsColumn::Name, Order::Desc),
				_ => query.order_by(EventsColumn::Name, Order::Asc),
			},
			_ => match params.sort_direction {
				Some(SortDirection::Asc) => {
					query.order_by(EventsColumn::CreatedAt, Order::Asc)
				}
				_ => query.order_by(EventsColumn::CreatedAt, Order::Desc),
			},
		};

		let paginator = query.paginate(self.db.as_ref(), per_page as u64);
		let total = paginator
			.num_items()
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		let events = paginator
			.fetch_page((page - 1) as u64)
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		let data = events.into_iter().map(to_entity).collect();
		let meta = PaginatorResponseMeta::new(page, per_page, total as u32);
		Ok(PaginatorResponse { data, meta })
	}

	async fn find_by_id(&self, id: Uuid) -> Result<EventEntity, AppError> {
		let event = EventsEntity::find_by_id(id)
			.filter(EventsColumn::IsDeleted.eq(false))
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?
			.ok_or_else(|| AppError::NotFoundError("Event not found".to_string()))?;

		Ok(to_entity(event))
	}

	async fn create(&self, entity: EventEntity) -> Result<(), AppError> {
		let active_model = EventsActiveModel {
			id: ActiveValue::Set(entity.id),
			name: ActiveValue::Set(entity.name),
			description: ActiveValue::Set(entity.description),
			detail_link: ActiveValue::Set(entity.detail_link),
			price: ActiveValue::Set(entity.price),
			is_online: ActiveValue::Set(entity.is_online),
			is_deleted: ActiveValue::Set(false),
			location: ActiveValue::Set(entity.location),
			start_date: ActiveValue::Set(entity.start_date),
			end_date: ActiveValue::Set(entity.end_date),
			created_at: ActiveValue::Set(chrono::Utc::now()),
			updated_at: ActiveValue::Set(chrono::Utc::now()),
		};

		EventsEntity::insert(active_model)
			.exec(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		Ok(())
	}

	async fn update(&self, entity: EventEntity) -> Result<(), AppError> {
		let mut active_model: EventsActiveModel = EventsEntity::find_by_id(entity.id)
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?
			.ok_or_else(|| AppError::NotFoundError("Event not found".to_string()))?
			.into();

		active_model.name = ActiveValue::Set(entity.name);
		active_model.description = ActiveValue::Set(entity.description);
		active_model.detail_link = ActiveValue::Set(entity.detail_link);
		active_model.price = ActiveValue::Set(entity.price);
		active_model.is_online = ActiveValue::Set(entity.is_online);
		active_model.location = ActiveValue::Set(entity.location);
		active_model.start_date = ActiveValue::Set(entity.start_date);
		active_model.end_date = ActiveValue::Set(entity.end_date);
		active_model.updated_at = ActiveValue::Set(chrono::Utc::now());

		active_model
			.update(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(())
	}

	async fn delete(&self, id: Uuid) -> Result<(), AppError> {
		let mut active_model: EventsActiveModel = EventsEntity::find_by_id(id)
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?
			.ok_or_else(|| AppError::NotFoundError("Event not found".to_string()))?
			.into();

		active_model.is_deleted = ActiveValue::Set(true);
		active_model.updated_at = ActiveValue::Set(chrono::Utc::now());
		active_model
			.update(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(())
	}
}
