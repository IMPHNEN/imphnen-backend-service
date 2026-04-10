use super::handlers::{
	delete_roadmap, get_roadmap_by_id, get_roadmap_list, patch_update_roadmap,
	post_create_roadmap, post_vote_roadmap,
};
use crate::roadmap::application::RoadmapServiceImpl;
use crate::roadmap::domain::RoadmapService;
use crate::roadmap::infrastructure::persistence::PostgresRoadmapRepository;
use axum::{
	Extension, Router,
	routing::{delete, get, patch, post},
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

fn build_service(db: DatabaseConnection) -> Arc<dyn RoadmapService> {
	let repo = Arc::new(PostgresRoadmapRepository::new(db));
	Arc::new(RoadmapServiceImpl::new(repo))
}

pub fn roadmap_public_routes(db: DatabaseConnection) -> Router {
	let service = build_service(db);
	Router::new()
		.route("/roadmap", get(get_roadmap_list))
		.route("/roadmap/detail/{id}", get(get_roadmap_by_id))
		.route("/roadmap/vote/{id}", post(post_vote_roadmap))
		.layer(Extension(service))
}

pub fn roadmap_protected_routes(db: DatabaseConnection) -> Router {
	let service = build_service(db);
	Router::new()
		.route("/roadmap/create", post(post_create_roadmap))
		.route("/roadmap/update/{id}", patch(patch_update_roadmap))
		.route("/roadmap/delete/{id}", delete(delete_roadmap))
		.layer(Extension(service))
}
