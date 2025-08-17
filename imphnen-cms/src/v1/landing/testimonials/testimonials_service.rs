use super::{
	testimonials_dto::{
		TestimonialsCreateRequestDto, TestimonialsDetailItemDto,
		TestimonialsListItemDto, TestimonialsUpdateRequestDto,
	},
	testimonials_repository::TestimonialsRepository,
	testimonials_schema::TestimonialsSchema,
};
use axum::{http::StatusCode, response::Response};
use imphnen_libs::{
	AppState, MetaRequestDto, ResponseListSuccessDto, ResponseSuccessDto,
};
use imphnen_utils::{
	common_response, success_list_response, success_response, success_created_response, validate_request,
};

pub struct TestimonialsService;

impl TestimonialsService {
	pub async fn get_testimonial_list(
		state: &AppState,
		meta: MetaRequestDto,
	) -> Response {
		let repo = TestimonialsRepository::new(state);
		match repo.query_testimonial_list(meta).await {
			Ok(data) => {
				let items: Vec<TestimonialsListItemDto> = data
					.data
					.into_iter()
					.filter(|e| !e.is_deleted)
					.map(|e| e.from())
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

	pub async fn get_testimonial_by_id(state: &AppState, id: String) -> Response {
		let repo = TestimonialsRepository::new(state);
		match repo.query_testimonial_by_id(id).await {
			Ok(testimonial) if !testimonial.is_deleted => {
				success_response(ResponseSuccessDto {
					data: TestimonialsDetailItemDto {
						id: testimonial.id.to_raw(),
						user_id: testimonial.user.id.to_raw(),
						user_fullname: testimonial.user.fullname,
						role: testimonial.role,
						content: testimonial.content,
						created_at: testimonial.created_at,
						updated_at: testimonial.updated_at,
					},
				})
			}
			Ok(_) => common_response(StatusCode::NOT_FOUND, "Testimonial not found"),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	pub async fn create_testimonial(
		state: &AppState,
		payload: TestimonialsCreateRequestDto,
		authenticated_user: &imphnen_iam::UsersDetailQueryDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = TestimonialsRepository::new(state);
		let schema = TestimonialsSchema::create(payload, &authenticated_user.id);
		match repo.query_create_testimonial(schema).await {
			Ok(created_testimonial) => {
				success_created_response(ResponseSuccessDto {
					data: TestimonialsDetailItemDto {
						id: created_testimonial.id.to_raw(),
						user_id: created_testimonial.user.id.to_raw(),
						user_fullname: authenticated_user.fullname.clone(),
						role: created_testimonial.role,
						content: created_testimonial.content,
						created_at: created_testimonial.created_at,
						updated_at: created_testimonial.updated_at,
					},
				})
			}
			Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
		}
	}

	pub async fn update_testimonial(
		state: &AppState,
		id: String,
		payload: TestimonialsUpdateRequestDto,
		authenticated_user: &imphnen_iam::UsersDetailQueryDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = TestimonialsRepository::new(state);
		let schema = TestimonialsSchema::update(payload, id, &authenticated_user.id);
		match repo.query_update_testimonial(schema).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	pub async fn delete_testimonial(
		state: &AppState,
		id: String,
		_authenticated_user: &imphnen_iam::UsersDetailQueryDto,
	) -> Response {
		let repo = TestimonialsRepository::new(state);
		match repo.query_delete_testimonial(id).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}
}
