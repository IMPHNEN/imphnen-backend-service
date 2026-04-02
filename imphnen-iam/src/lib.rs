pub mod permission_macros;
pub mod permissions_guard;

pub mod auth;
pub mod permissions;
pub mod roles;
pub mod users;

pub use auth::auth_public_routes;
pub use permissions::{permissions_protected_routes, permissions_public_routes};
pub use roles::{roles_protected_routes, roles_public_routes};
pub use users::{users_protected_routes, users_public_routes};

pub use imphnen_entities::{
	MessageResponseDto, PermissionsEnum, PermissionsItemDto, PermissionsQueryDto,
	ResponseListSuccessDto, ResponseSuccessDto, UsersDetailQueryDto,
};

pub use imphnen_libs::{
	AppState, AuthRepositoryTrait, Env, UserLookupService, decode_access_token,
	decode_refresh_token, encode_access_token, encode_refresh_token,
	encode_reset_password_token, hash_password, jsonwebtoken::Claims, verify_password,
};

pub use imphnen_email::send_email;
pub use imphnen_libs::AppStatePostgresExt;
pub use imphnen_utils::{
	csrf_token::generate_oauth_csrf_token, csrf_token::validate_csrf_token,
	csrf_token::validate_oauth_csrf_token, errors::AppError,
	extract_email::extract_email_async, generate_date::get_iso_date,
	generate_otp::OtpManager, response_format::ApiCreated,
	response_format::ApiMessage, response_format::ApiPaginated,
	response_format::ApiSuccess,
};
pub use paginator_axum::PaginationQuery;
pub use paginator_rs::PaginationParams;
pub use paginator_utils::{PaginatorResponse, PaginatorResponseMeta};

pub use permissions_guard::permissions_guard;
