use std::sync::Arc;
use axum::{Router, routing::{delete, get, patch, post}, Extension};
use sea_orm::DatabaseConnection;
use crate::testimonials::application::TestimonialServiceImpl;
use crate::testimonials::domain::TestimonialService;
use crate::testimonials::infrastructure::persistence::PostgresTestimonialRepository;
use super::handlers::{
    delete_testimonial, get_testimonial_by_id, get_testimonial_list,
    patch_update_testimonial, post_create_testimonial,
};

fn build_service(db: DatabaseConnection) -> Arc<dyn TestimonialService> {
    let repo = Arc::new(PostgresTestimonialRepository::new(db));
    Arc::new(TestimonialServiceImpl::new(repo))
}

pub fn testimonials_public_routes(db: DatabaseConnection) -> Router {
    let service = build_service(db);
    Router::new()
        .route("/cms/landing/testimonials", get(get_testimonial_list))
        .route("/cms/landing/testimonials/detail/{id}", get(get_testimonial_by_id))
        .layer(Extension(service))
}

pub fn testimonials_protected_routes(db: DatabaseConnection) -> Router {
    let service = build_service(db);
    Router::new()
        .route("/cms/landing/testimonials/create", post(post_create_testimonial))
        .route("/cms/landing/testimonials/update/{id}", patch(patch_update_testimonial))
        .route("/cms/landing/testimonials/delete/{id}", delete(delete_testimonial))
        .layer(Extension(service))
}
