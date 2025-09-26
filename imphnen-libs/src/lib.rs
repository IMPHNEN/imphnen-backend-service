/*!
# imphnen-libs

A collection of utility libraries and services for the imphnen project, providing integrations
with various external services and common functionality.

This crate includes modules for:
- Password hashing with Argon2 (`argon`)
- Axum web framework utilities (`axum`)
- Environment configuration (`environment`)
- JWT token handling (`jsonwebtoken`)
- Email sending with Lettre (`lettre`)
- MinIO object storage client (`minio`)
- Service abstractions (`services`)
- SurrealDB database client (`surrealdb`)
*/

use std::sync::Arc;

pub mod argon;
pub mod axum;
pub mod environment;
pub mod jsonwebtoken;
pub mod lettre;
pub mod minio;
pub mod services;
pub mod surrealdb;

pub use argon::{hash_password, verify_password};
pub use axum::axum_init;
pub use environment::{ENV, Env};
pub use imphnen_entities::{
    MessageResponseDto,
    MetaRequestDto,
    MetaResponseDto,
    ResponseSuccessDto,
    ResponseListSuccessDto,
    CountResult,
    Error,
    ExperienceDto,
    EducationDto,
    UsersDetailQueryDto,
    PermissionsEnum,
    PermissionsItemDto,
    PermissionsQueryDto,
};
pub use jsonwebtoken::{
    Claims, encode_access_token, encode_refresh_token, decode_access_token,
    decode_refresh_token, encode_reset_password_token, generate_jwt
};
pub use lettre::send_email;
pub use minio::*; // Minio has many useful exports, keeping for now
pub use services::{UserLookupService, AuthRepositoryTrait};
pub use surrealdb::{
    surrealdb_init_ws, surrealdb_init_mem, SurrealWsClient, SurrealMemClient,
    ResourceEnum
};

#[derive(Clone)]
pub struct AppState {
	pub surrealdb_ws: SurrealWsClient,
	pub surrealdb_mem: SurrealMemClient,
	pub user_lookup_service: Arc<dyn UserLookupService>,
	pub auth_repository: Arc<dyn AuthRepositoryTrait>,
}
