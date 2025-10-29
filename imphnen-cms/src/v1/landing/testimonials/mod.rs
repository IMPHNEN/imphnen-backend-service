use axum::{
	Router,
	routing::{delete, get, patch, post},
};

pub mod testimonials_controller;
pub mod testimonials_dto;
pub mod testimonials_repository;
pub mod testimonials_schema;
pub mod testimonials_service;

// Export only the necessary public items
pub use testimonials_dto::{
	TestimonialsCreateRequestDto,
	TestimonialsUpdateRequestDto,
	TestimonialsListItemDto,
	TestimonialsDetailItemDto,
};
pub use testimonials_controller::{
	get_testimonial_list,
	get_testimonial_by_id,
	post_create_testimonial,
	patch_update_testimonial,
	delete_testimonial,
};

pub fn testimonials_public_routes() -> Router {
	Router::new()
		.route("/cms/landing/testimonials", get(get_testimonial_list))
		.route("/cms/landing/testimonials/detail/{id}", get(get_testimonial_by_id))
}

pub fn testimonials_protected_routes() -> Router {
	Router::new()
		.route("/cms/landing/testimonials/create", post(post_create_testimonial))
		.route("/cms/landing/testimonials/update/{id}", patch(patch_update_testimonial))
		.route("/cms/landing/testimonials/delete/{id}", delete(delete_testimonial))
}
