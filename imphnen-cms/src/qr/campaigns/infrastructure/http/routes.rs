use axum::{
	Extension, Router,
	middleware::from_fn,
	routing::{delete, post, put},
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::qr::{
	campaigns::{
		application::campaign_service::QrCampaignServiceImpl,
		domain::{repository::CampaignRepository, service::QrCampaignService},
		infrastructure::{
			http::handlers::{
				activate_campaign_handler, create_campaign_handler, delete_campaign_handler,
				list_campaigns_handler, process_image_handler,
			},
			persistence::postgres_campaign_repository::PostgresCampaignRepository,
		},
	},
	middleware::qr_auth::qr_auth_middleware,
};

pub fn qr_campaigns_routes(pool: Arc<PgPool>) -> Router {
	let repo: Arc<dyn CampaignRepository> =
		Arc::new(PostgresCampaignRepository::new(pool.clone()));
	let service: Arc<dyn QrCampaignService> =
		Arc::new(QrCampaignServiceImpl::new(repo));

	Router::new()
		.route(
			"/campaigns",
			post(create_campaign_handler).get(list_campaigns_handler),
		)
		.route("/campaigns/{id}/activate", put(activate_campaign_handler))
		.route("/campaigns/{id}", delete(delete_campaign_handler))
		.route("/campaigns/process-image", post(process_image_handler))
		.layer(Extension(service))
		.layer(Extension(pool))
		.layer(from_fn(qr_auth_middleware))
}
