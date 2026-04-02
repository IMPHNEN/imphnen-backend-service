use super::handlers::get_certificate_handler;
use crate::certificates::application::certificate_service::CertificateServiceImpl;
use crate::certificates::domain::service::CertificateService;
use crate::certificates::infrastructure::persistence::PostgresCertificateRepository;
use axum::{Extension, Router, routing::get};
use sqlx::PgPool;
use std::sync::Arc;

pub fn hackathon_certificates_routes(pool: Arc<PgPool>) -> Router {
	let service: Arc<dyn CertificateService> = Arc::new(CertificateServiceImpl::new(
		Arc::new(PostgresCertificateRepository::new(pool.clone())),
	));
	Router::new()
		.route("/certificates/{user_id}", get(get_certificate_handler))
		.layer(Extension(service))
		.layer(Extension(pool))
}
