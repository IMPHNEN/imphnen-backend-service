use axum::{
	Router,
	routing::{delete, get, patch, post},
};

pub mod testimonials_controller;
pub mod testimonials_dto;
pub mod testimonials_repository;
pub mod testimonials_schema;
pub mod testimonials_service;

pub use testimonials_controller::*;
pub use testimonials_dto::*;
pub use testimonials_repository::*;
pub use testimonials_schema::*;
pub use testimonials_service::*;

pub fn testimonials_public_routes() -> Router {
	Router::new()
		.route(
			"/cms/landing/testimonials",
			get(testimonials_controller::get_testimonial_list),
		)
		.route(
			"/cms/landing/testimonials/detail/{id}",
			get(testimonials_controller::get_testimonial_by_id),
		)
}

pub fn testimonials_protected_routes() -> Router {
	Router::new()
		.route(
			"/cms/landing/testimonials/create",
			post(testimonials_controller::post_create_testimonial),
		)
		.route(
			"/cms/landing/testimonials/update/{id}",
			patch(testimonials_controller::patch_update_testimonial),
		)
		.route(
			"/cms/landing/testimonials/delete/{id}",
			delete(testimonials_controller::delete_testimonial),
		)
}
