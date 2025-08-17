use super::{events_dto::EventsQueryDto, events_schema::EventsSchema};
use anyhow::{Result, bail};
use imphnen_libs::{AppState, MetaRequestDto, ResourceEnum, ResponseListSuccessDto};
use imphnen_utils::{DetailQueryBuilder, ListQueryBuilder, get_id, get_iso_date, make_thing};
use std::time::Instant;
use tracing::instrument;
use tracing::info;

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
		let query = ListQueryBuilder::new(ResourceEnum::Events.to_string())
			.with_select_fields(vec!["*"])
			.with_pagination(meta.page, Some(10))
			.with_sorting(meta.sort_by.as_deref(), meta.order.as_deref())
			.build();
		info!(query = %query, "Executing SurrealDB query");
		let res: Vec<EventsQueryDto> =
			self.state.surrealdb_ws.query(query).await?.take(0)?;
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
	pub async fn query_event_by_id(&self, id: String) -> Result<EventsQueryDto> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
	      // Attempt to parse the ID. If it's a full Thing (e.g., "events:some_id"), extract the ID part.
	      // Otherwise, assume it's already the raw ID.
	      let parsed_id = if id.contains(":") {
	          let thing = make_thing(ResourceEnum::Events.to_string().as_str(), &id);
	          get_id(&thing)?.1.to_string()
	      } else {
	          id.clone()
	      };

		let builder = DetailQueryBuilder::new(ResourceEnum::Events.to_string())
			.with_id(&parsed_id)
			.with_select_fields(vec!["*"]);
		let sql = builder.build();
		info!(query = %sql, "Executing SurrealDB query");
		let result: Option<EventsQueryDto> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_event_by_id' took: {elapsed:.2?}");
		}

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

	#[instrument(skip(self, data), err)]
	pub async fn query_create_event(&self, data: EventsSchema) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let query_str = format!("CREATE {} CONTENT ...", ResourceEnum::Events.to_string());
		info!(query = %query_str, "Executing SurrealDB query");
		let record: Option<EventsSchema> = db
			.create(ResourceEnum::Events.to_string())
			.content(data)
			.await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_create_event' took: {elapsed:.2?}");
		}

		match record {
			Some(_) => Ok("Success create event".into()),
			None => bail!("Failed to create event"),
		}
	}

	#[instrument(skip(self, data), err)]
	pub async fn query_update_event(&self, data: EventsSchema) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;

		let existing = self.query_event_by_id(data.id.id.to_raw()).await?;
		if existing.is_deleted {
			bail!("Event already deleted");
		}

		let merged = EventsSchema {
			created_at: existing.created_at,
			updated_at: get_iso_date(),
			..data
		};

		let record_key = get_id(&merged.id)?;
		let query_str = format!("UPDATE {:?} MERGE ...", record_key);
		info!(query = %query_str, "Executing SurrealDB query");
		let record: Option<EventsSchema> = db.update(record_key).merge(merged).await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_update_event' took: {elapsed:.2?}");
		}

		match record {
			Some(_) => Ok("Success update event".into()),
			None => bail!("Failed to update event"),
		}
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_delete_event(&self, id: String) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let event = self.query_event_by_id(id).await?;
		if event.is_deleted {
			bail!("Event not found");
		}

		let record_key = get_id(&event.id)?;
		let query_str = format!("UPDATE {:?} MERGE {{ is_deleted: true }}", record_key);
		info!(query = %query_str, "Executing SurrealDB query");
		let record: Option<EventsSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_delete_event' took: {elapsed:.2?}");
		}

		match record {
			Some(_) => Ok("Success delete event".into()),
			None => bail!("Failed to delete event"),
		}
	}
}
