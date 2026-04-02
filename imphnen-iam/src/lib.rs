pub mod permission_macros;
pub mod permissions_guard;

pub mod permissions;
pub mod roles;
pub mod users;
pub mod auth;

pub use permissions::{permissions_public_routes, permissions_protected_routes};
pub use roles::{roles_public_routes, roles_protected_routes};
pub use users::{users_public_routes, users_protected_routes};
pub use auth::auth_public_routes;

pub use imphnen_entities::{
    MessageResponseDto,
    ResponseSuccessDto,
    ResponseListSuccessDto,
    UsersDetailQueryDto,
    PermissionsEnum,
    PermissionsItemDto,
    PermissionsQueryDto,
};

// Explicitly export only the imphnen_libs types actually used in IAM
pub use imphnen_libs::{
    AppState,
    decode_access_token,
    decode_refresh_token,
    encode_access_token,
    encode_refresh_token,
    encode_reset_password_token,
    hash_password,
    send_email,
    verify_password,
    Env,
    UserLookupService,
    AuthRepositoryTrait,
    jsonwebtoken::Claims,
};

pub use imphnen_utils::{
    response_format::ApiSuccess,
    response_format::ApiCreated,
    response_format::ApiPaginated,
    response_format::ApiMessage,
    csrf_token::generate_oauth_csrf_token,
    csrf_token::validate_oauth_csrf_token,
    csrf_token::validate_csrf_token,
    extract_email::extract_email_async,
    generate_otp::OtpManager,
    errors::AppError,
    generate_date::get_iso_date,
};
pub use paginator_axum::PaginationQuery;
pub use paginator_rs::PaginationParams;
pub use paginator_utils::{PaginatorResponse, PaginatorResponseMeta};
pub use imphnen_libs::AppStatePostgresExt;

pub use permissions_guard::permissions_guard;

