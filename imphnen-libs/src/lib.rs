use std::sync::Arc;

pub mod argon;
pub mod axum;
pub mod enviroment;
pub mod jsonwebtoken;
pub mod lettre;
pub mod minio;
pub mod services;
pub mod surrealdb;

pub use argon::*;
pub use axum::*;
pub use enviroment::*;
pub use imphnen_entities::*;
pub use jsonwebtoken::*;
pub use lettre::*;
pub use minio::*;
pub use services::*;
pub use surrealdb::*;

#[derive(Clone)]
pub struct AppState {
	pub surrealdb_ws: SurrealWsClient,
	pub surrealdb_mem: SurrealMemClient,
	pub user_lookup_service: Arc<dyn UserLookupService>,
	pub auth_repository: Arc<dyn AuthRepositoryTrait>,
}
