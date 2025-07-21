use crate::{
	AppState, GachaItemDto, GachaItemRepository, GachaItemRequestDto, GachaItemSchema,
	MetaRequestDto, ResourceEnum, ResponseListSuccessDto, ResponseSuccessDto,
	common_response, make_thing, success_list_response, success_response,
	validate_request,
};
use axum::http::StatusCode;
use axum::response::Response;

pub struct GachaItemService;

impl GachaItemService {
	pub async fn get_gacha_item_list(
		state: &AppState,
		meta: MetaRequestDto,
	) -> Response {
		let repo = GachaItemRepository::new(state);
		match repo.query_gacha_item_list(meta).await {
			Ok(data) => {
				let response = ResponseListSuccessDto {
					data: data.data,
					meta: data.meta,
				};
				success_list_response(response)
			}
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	pub async fn get_gacha_item_by_id(state: &AppState, id: String) -> Response {
		let repo = GachaItemRepository::new(state);
		match repo.query_gacha_item_by_id(id).await {
			Ok(item) => success_response(ResponseSuccessDto {
				data: GachaItemDto::from(item),
			}),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	pub async fn create_gacha_item(
		state: &AppState,
		payload: GachaItemRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = GachaItemRepository::new(state);
		let schema = GachaItemSchema {
			id: make_thing(&ResourceEnum::GachaItems.to_string(), &payload.name), // Fixed: Use payload.name or some other identifier
			name: payload.name,
			image_url: payload.image_url,
			..Default::default()
		};
		match repo.query_create_gacha_item(schema).await {
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
		}
	}

	pub async fn update_gacha_item(
		state: &AppState,
		payload: GachaItemRequestDto,
		id: String,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = GachaItemRepository::new(state);
		let schema = GachaItemSchema {
			id: make_thing(&ResourceEnum::GachaItems.to_string(), &id),
			name: payload.name,
			image_url: payload.image_url,
			..Default::default()
		};
		match repo.query_update_gacha_item(schema).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => {
				if e.to_string().contains("not found") {
					common_response(StatusCode::NOT_FOUND, "Gacha Item not found")
				} else {
					common_response(StatusCode::BAD_REQUEST, &e.to_string())
				}
			}
		}
	}

	pub async fn delete_gacha_item(state: &AppState, id: String) -> Response {
		let repo = GachaItemRepository::new(state);
		match repo.query_delete_gacha_item(id).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => {
				if e.to_string().contains("not found") {
					common_response(StatusCode::NOT_FOUND, "Gacha Item not found")
				} else {
					common_response(StatusCode::BAD_REQUEST, &e.to_string())
				}
			}
		}
	}
}
