use super::{
	events_dto::{
		EventsCreateRequestDto, EventsDetailItemDto, EventsListItemDto, EventsQueryDto,
		EventsUpdateRequestDto,
	},
	events_repository::EventsRepository,
	events_schema::EventsSchema,
};
use axum::{http::StatusCode, response::Response};
use imphnen_libs::{
	AppState, MetaRequestDto, ResponseListSuccessDto, ResponseSuccessDto,
};
use imphnen_utils::{
	common_response, success_list_response, success_response, validate_request,
};

pub struct EventsService;

impl EventsService {
	pub async fn get_event_list(state: &AppState, meta: MetaRequestDto) -> Response {
		let repo = EventsRepository::new(state);
		match repo.query_event_list(meta).await {
			Ok(data) => {
				let items: Vec<EventsListItemDto> = data
					.data
					.into_iter()
					.filter(|e| !e.is_deleted)
					.map(EventsQueryDto::from)
					.collect();
				let response = ResponseListSuccessDto {
					data: items,
					meta: data.meta,
				};
				success_list_response(response)
			}
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	pub async fn get_event_by_id(state: &AppState, id: String) -> Response {
		let repo = EventsRepository::new(state);
		match repo.query_event_by_id(id).await {
			Ok(event) if !event.is_deleted => success_response(ResponseSuccessDto {
				data: EventsDetailItemDto {
					id: event.id.id.to_raw(),
					name: event.name,
					description: event.description,
					detail_link: event.detail_link,
					price: event.price,
					is_online: event.is_online,
					start_date: event.start_date,
					end_date: event.end_date,
					created_at: event.created_at,
					updated_at: event.updated_at,
					location: event.location,
				},
			}),
			Ok(_) => common_response(StatusCode::NOT_FOUND, "Event not found"),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	pub async fn create_event(
		state: &AppState,
		payload: EventsCreateRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = EventsRepository::new(state);
		let schema = EventsSchema::create(payload);
		match repo.query_create_event(schema).await {
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
		}
	}

	pub async fn update_event(
		state: &AppState,
		id: String,
		payload: EventsUpdateRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = EventsRepository::new(state);
		let schema = EventsSchema::update(payload, id);
		match repo.query_update_event(schema).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	pub async fn delete_event(state: &AppState, id: String) -> Response {
		let repo = EventsRepository::new(state);
		match repo.query_delete_event(id).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}
}
