use crate::AppState;
use imphnen_entities::{MetaRequestDto, ResponseListSuccessDto, ResponseSuccessDto};
use imphnen_utils::{common_response, make_thing, success_list_response, success_response};
use crate::v1::gacha_items::GachaItemDto;
use crate::v1::gacha_items::gacha_items_dto::{GachaItemRequestDto, GachaItemUpdateRequestDto};
use crate::v1::gacha_items::gacha_items_repository::GachaItemRepository;
use crate::v1::gacha_items::gacha_items_schema::GachaItemSchema;
use imphnen_libs::ResourceEnum;
use axum::http::StatusCode;
use axum::response::Response;
use imphnen_utils::get_iso_date;

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
		// Validation is now automatic via ValidatedJson extractor
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
		payload: GachaItemUpdateRequestDto,
		id: String,
	) -> Response {
		// Validation is now automatic via ValidatedJson extractor
		let repo = GachaItemRepository::new(state);
		
		// Get current gacha item data first
		let _thing_id = make_thing(&ResourceEnum::GachaItems.to_string(), &id);
		let current_item = match repo.query_gacha_item_by_id(id.clone()).await {
			Ok(item) => item,
			Err(_) => return common_response(StatusCode::NOT_FOUND, "Gacha Item not found"),
		};
		
		let mut updated_item = current_item;
		updated_item.updated_at = Some(get_iso_date());
		
		// Only update fields that are provided
		if let Some(name) = payload.name {
			updated_item.name = name;
		}
		if let Some(image_url) = payload.image_url {
			updated_item.image_url = image_url;
		}
		
		match repo.query_update_gacha_item(updated_item).await {
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
