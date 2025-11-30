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
- PostgreSQL database client (`postgres`)
- Dual-mode repository pattern (`dual_mode_repository`)
*/

use std::sync::Arc;

pub mod postgres;
pub mod argon;
pub mod axum;
pub mod environment;
pub mod jsonwebtoken;
pub mod lettre;
pub mod minio;
pub mod services;
pub mod dual_mode_repository;

pub use argon::{hash_password, verify_password};
pub use axum::{axum_init, ValidatedJson};
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
pub use minio::{
    MinioConfig, MinioService, UploadResult, FileType, UploadRequest, FileMetadata,
    create_minio_service_from_config, decode_base64_file, extract_content_type_from_data_url
};
pub use services::{UserLookupService, AuthRepositoryTrait};
// Re-export concrete Postgres service implementations for convenience
pub use services::PostgresUserLookupService;
pub use services::PostgresAuthRepository;
pub use postgres::{
    PostgresConnection, PostgresConfig, PostgresError, AppStatePostgresExt,
};
pub use dual_mode_repository::{PostgresRepository, PostgresRepositoryDefaultImpl, conversion_utils, PostgresRepositoryError};

#[derive(Clone)]
pub struct AppState {
	pub postgres_connection: Arc<PostgresConnection>,
	pub user_lookup_service: Arc<dyn UserLookupService>,
	pub auth_repository: Arc<dyn AuthRepositoryTrait>,
}

impl AppState {
	/// Create a new AppState with PostgreSQL connection
	pub async fn new(
		postgres_config: PostgresConfig,
		user_lookup_service: Arc<dyn UserLookupService>,
		auth_repository: Arc<dyn AuthRepositoryTrait>,
	) -> Result<Self, PostgresError> {
		let postgres_connection = PostgresConnection::new(postgres_config).await?;
		
		Ok(Self {
			postgres_connection: Arc::new(postgres_connection),
			user_lookup_service,
			auth_repository,
		})
	}
}

impl AppStatePostgresExt for AppState {
	fn postgres_connection(&self) -> &PostgresConnection {
		&self.postgres_connection
	}
}
