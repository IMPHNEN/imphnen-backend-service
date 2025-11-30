use sea_orm::prelude::*;
use sea_orm::{ActiveValue, QueryOrder};
use std::time::Instant;
use tracing::instrument;
use uuid::Uuid; // Added Uuid import
use imphnen_entities::seaorm::common::events::{Entity as EventsEntity, Column as EventsColumn, Model as EventsModel};
use imphnen_libs::{AppState, MetaRequestDto, ResponseListSuccessDto, AppStatePostgresExt};
use imphnen_utils::AppError;
use imphnen_utils::Result;
use crate::events::events_dto::EventsQueryDto;
use crate::events::events_schema::EventsSchema;

pub struct EventsRepository<'a> {
	state: &'a AppState,
}

impl<'a> EventsRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	#[instrument(skip(self, meta), err)]
	pub async fn query_event_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<EventsQueryDto>>> {
		let now = Instant::now();
		let page = meta.page.unwrap_or(1);
		let page_size = 10u64;
		let _offset = (page - 1) * page_size;
		
            let mut query = EventsEntity::find()
                    .filter(EventsColumn::IsDeleted.eq(false));		// Add sorting
		if let Some(sort_by) = &meta.sort_by {
			match sort_by.as_str() {
				"created_at" => {
					if meta.order.as_deref() == Some("desc") {
						query = query.order_by_desc(EventsColumn::CreatedAt);
					} else {
						query = query.order_by_asc(EventsColumn::CreatedAt);
					}
				}
				"name" => {
					if meta.order.as_deref() == Some("desc") {
						query = query.order_by_desc(EventsColumn::Name);
					} else {
						query = query.order_by_asc(EventsColumn::Name);
					}
				}
				_ => {
					query = query.order_by_desc(EventsColumn::CreatedAt);
				}
			}
		} else {
			query = query.order_by_desc(EventsColumn::CreatedAt);
		}
		
		let paginator = query.paginate(self.state.postgres_db(), page_size);
		let events: Vec<EventsModel> = paginator.fetch_page(page - 1).await?;
		
		let res: Vec<EventsQueryDto> = events.into_iter().map(|model| EventsQueryDto {
			id: model.id.to_string(),
			name: model.name,
			description: model.description,
			detail_link: model.detail_link,
			price: model.price,
			is_online: model.is_online,
			is_deleted: model.is_deleted,
			start_date: model.start_date.to_rfc3339(),
			end_date: model.end_date.to_rfc3339(),
			created_at: model.created_at.to_rfc3339(),
			updated_at: model.updated_at.to_rfc3339(),
			location: model.location,
		}).collect();
		
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_event_list' took: {elapsed:.2?}");
		}
		let data = ResponseListSuccessDto {
			data: res,
			meta: None,
		};
		Ok(data)
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_event_by_id(&self, id: Uuid) -> Result<EventsQueryDto> {
		let now = Instant::now();
		
		let event = EventsEntity::find_by_id(id)
			.filter(EventsColumn::IsDeleted.eq(false))
			.one(self.state.postgres_db())
			.await?
			.ok_or_else(|| anyhow::anyhow!("Event not found"))?;
		
		let result = EventsQueryDto {
			id: event.id.to_string(),
			name: event.name,
			description: event.description,
			detail_link: event.detail_link,
			price: event.price,
			is_online: event.is_online,
			is_deleted: event.is_deleted,
			start_date: event.start_date.to_rfc3339(),
			end_date: event.end_date.to_rfc3339(),
			created_at: event.created_at.to_rfc3339(),
			updated_at: event.updated_at.to_rfc3339(),
			location: event.location,
		};
		
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_event_by_id' took: {elapsed:.2?}");
		}

		Ok(result)
	}

	#[instrument(skip(self, data), err)]
	pub async fn query_create_event(&self, data: EventsSchema) -> Result<String> {
		let now = Instant::now();
		
		let active_model = imphnen_entities::seaorm::common::events::ActiveModel {
			id: ActiveValue::Set(Uuid::parse_str(&data.id)?),
			name: ActiveValue::Set(data.name),
			description: ActiveValue::Set(data.description),
			detail_link: ActiveValue::Set(data.detail_link),
			price: ActiveValue::Set(data.price),
			is_online: ActiveValue::Set(data.is_online),
			is_deleted: ActiveValue::Set(data.is_deleted),
			location: ActiveValue::Set(data.location),
			start_date: ActiveValue::Set(chrono::DateTime::parse_from_rfc3339(&data.start_date)?.with_timezone(&chrono::Utc)),
			end_date: ActiveValue::Set(chrono::DateTime::parse_from_rfc3339(&data.end_date)?.with_timezone(&chrono::Utc)),
			created_at: ActiveValue::Set(chrono::Utc::now()),
			updated_at: ActiveValue::Set(chrono::Utc::now()),
		};
		
		let _result = EventsEntity::insert(active_model).exec(self.state.postgres_db()).await?;
		
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_create_event' took: {elapsed:.2?}");
		}

		Ok("Success create event".into())
	}

	#[instrument(skip(self, data), err)]
	pub async fn query_update_event(&self, data: EventsSchema) -> Result<String> {
		let now = Instant::now();
		
		let existing = self.query_event_by_id(Uuid::parse_str(&data.id)?).await?;
		if existing.is_deleted {
			return Err(AppError::BadRequestError("Event already deleted".to_string()));
		}
		
		let mut active_model: imphnen_entities::seaorm::common::events::ActiveModel = EventsEntity::find_by_id(Uuid::parse_str(&data.id)?)
			.one(self.state.postgres_db())
			.await?
			.ok_or_else(|| anyhow::anyhow!("Event not found"))?
			.into();
		
		active_model.name = ActiveValue::Set(data.name);
		active_model.description = ActiveValue::Set(data.description);
		active_model.detail_link = ActiveValue::Set(data.detail_link);
		active_model.price = ActiveValue::Set(data.price);
		active_model.is_online = ActiveValue::Set(data.is_online);
		active_model.location = ActiveValue::Set(data.location);
		active_model.start_date = ActiveValue::Set(chrono::DateTime::parse_from_rfc3339(&data.start_date)?.with_timezone(&chrono::Utc));
		active_model.end_date = ActiveValue::Set(chrono::DateTime::parse_from_rfc3339(&data.end_date)?.with_timezone(&chrono::Utc));
		active_model.updated_at = ActiveValue::Set(chrono::Utc::now());
		
		active_model.update(self.state.postgres_db()).await?;
		
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_update_event' took: {elapsed:.2?}");
		}

		Ok("Success update event".into())
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_delete_event(&self, id: Uuid) -> Result<String> {
		let now = Instant::now();
		let event = self.query_event_by_id(id).await?;
		if event.is_deleted {
			return Err(AppError::NotFoundError("Event not found".to_string()));
		}
		
		let mut active_model: imphnen_entities::seaorm::common::events::ActiveModel = EventsEntity::find_by_id(id)
			.one(self.state.postgres_db())
			.await?
			.ok_or_else(|| anyhow::anyhow!("Event not found"))?
			.into();
		
		active_model.is_deleted = ActiveValue::Set(true);
		active_model.updated_at = ActiveValue::Set(chrono::Utc::now());
		
		active_model.update(self.state.postgres_db()).await?;
		
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_delete_event' took: {elapsed:.2?}");
		}

		Ok("Success delete event".into())
	}
}
