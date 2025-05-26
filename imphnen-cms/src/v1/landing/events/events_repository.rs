use super::{events_dto::EventsQueryDto, events_schema::EventsSchema};
use anyhow::{Result, bail};
use imphnen_libs::{AppState, MetaRequestDto, ResourceEnum, ResponseListSuccessDto};
use imphnen_utils::{DetailQueryBuilder, ListQueryBuilder, get_id, get_iso_date};

pub struct EventsRepository<'a> {
	state: &'a AppState,
}

impl<'a> EventsRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_event_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<EventsQueryDto>>> {
		let query = ListQueryBuilder::new(&ResourceEnum::Events.to_string())
			.with_select_fields(vec!["*"])
			.with_pagination(meta.page, Some(10))
			.with_sorting(meta.sort_by.as_deref(), meta.order.as_deref())
			.build();
		let res: Vec<EventsQueryDto> =
			self.state.surrealdb_ws.query(query).await?.take(0)?;
		let data = ResponseListSuccessDto {
			data: res,
			meta: None,
		};
		Ok(data)
	}

	// Get event by ID
	pub async fn query_event_by_id(&self, id: String) -> Result<EventsQueryDto> {
		let db = &self.state.surrealdb_ws;
		let builder = DetailQueryBuilder::new(ResourceEnum::Events.to_string())
			.with_id(&id)
			.with_select_fields(vec!["*"]);
		let sql = builder.build();
		let result: Option<EventsQueryDto> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;

		match result {
			Some(event) => {
				if event.is_deleted {
					bail!("Event not found");
				}
				Ok(event)
			}
			None => bail!("Event not found"),
		}
	}

	// Create new event
	pub async fn query_create_event(&self, data: EventsSchema) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record: Option<EventsSchema> = db
			.create(ResourceEnum::Events.to_string())
			.content(data)
			.await?;

		match record {
			Some(_) => Ok("Success create event".into()),
			None => bail!("Failed to create event"),
		}
	}

	// Update existing event
	pub async fn query_update_event(&self, data: EventsSchema) -> Result<String> {
		let db = &self.state.surrealdb_ws;

		// Cek apakah event ada
		let existing = self.query_event_by_id(data.id.id.to_raw()).await?;
		if existing.is_deleted {
			bail!("Event already deleted");
		}

		// Merge field tertentu jika diperlukan
		let merged = EventsSchema {
			created_at: existing.created_at,
			updated_at: get_iso_date(),
			..data
		};

		let record_key = get_id(&merged.id)?;
		let record: Option<EventsSchema> = db.update(record_key).merge(merged).await?;

		match record {
			Some(_) => Ok("Success update event".into()),
			None => bail!("Failed to update event"),
		}
	}

	// Soft delete event (mark is_deleted = true)
	pub async fn query_delete_event(&self, id: String) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let event = self.query_event_by_id(id).await?;
		if event.is_deleted {
			bail!("Event not found");
		}

		let record_key = get_id(&event.id)?;
		let record: Option<EventsSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;

		match record {
			Some(_) => Ok("Success delete event".into()),
			None => bail!("Failed to delete event"),
		}
	}
}
