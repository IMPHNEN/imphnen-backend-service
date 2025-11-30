use crate::{
	AppState, MetaRequestDto, PermissionsRepository, PermissionsSchema, ResourceEnum,
	ResponseListSuccessDto, ResponseSuccessDto, common_response,
	success_list_response, success_response, validate_request,
};
use axum::http::StatusCode;
use axum::response::Response;
use crate::get_iso_date;
use imphnen_utils::make_thing;
use uuid::Uuid;

use super::{PermissionsRequestDto, PermissionsUpdateRequestDto};

pub struct PermissionsService;

impl PermissionsService {
	pub async fn get_permission_list(
		state: &AppState,
		meta: MetaRequestDto,
	) -> Response {
		let repo = PermissionsRepository::new(state);
		match repo.query_permission_list(meta).await {
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

	pub async fn get_permission_by_id(state: &AppState, id: String) -> Response {
		let repo = PermissionsRepository::new(state);
		match repo.transformed_query_permission_by_id(id).await {
			Ok(permission) => success_response(ResponseSuccessDto { data: permission }),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	pub async fn create_role(
		state: &AppState,
		payload: PermissionsRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = PermissionsRepository::new(state);
		match repo.query_permission_by_name(payload.name.clone()).await {
			Ok(_role) => {
				return common_response(
					StatusCode::CONFLICT,
					"Permission name already exists",
				);
			}
			Err(err) if err.to_string().contains("not found") => {}
			Err(e) => {
				return common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string());
			}
		}
		match repo
			.query_create_permission(PermissionsSchema {
				name: payload.name,
				..Default::default()
			})
			.await
		{
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
		}
	}

	pub async fn update_permission(
		state: &AppState,
		payload: PermissionsUpdateRequestDto,
		id: String,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = PermissionsRepository::new(state);
		
		// Get current permission data first
		let _thing_id = make_thing(&ResourceEnum::Permissions.to_string(), &id);
		let current_permission = match repo.query_permission_by_id(id.clone()).await {
			Ok(permission) => permission,
			Err(_) => return common_response(StatusCode::NOT_FOUND, "Permission not found"),
		};
		
		let mut updated_permission = current_permission;
		updated_permission.id = Uuid::parse_str(&id).unwrap_or_else(|_| Uuid::new_v4());
		updated_permission.updated_at = Some(get_iso_date());
		
		// Only update fields that are provided
		if let Some(name) = payload.name {
			updated_permission.name = name;
		}
		
		match repo.query_update_permission(updated_permission).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => {
				if e.to_string().contains("not found") {
					common_response(StatusCode::NOT_FOUND, "Permission not found")
				} else {
					common_response(StatusCode::BAD_REQUEST, &e.to_string())
				}
			}
		}
	}

	pub async fn delete_permission(state: &AppState, id: String) -> Response {
		let repo = PermissionsRepository::new(state);
		match repo.query_delete_permission(id).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => {
				if e.to_string().contains("not found") {
					common_response(StatusCode::NOT_FOUND, "Permission not found")
				} else {
					common_response(StatusCode::BAD_REQUEST, &e.to_string())
				}
			}
		}
	}
}
