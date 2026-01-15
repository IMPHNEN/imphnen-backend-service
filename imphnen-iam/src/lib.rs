pub mod v1;
pub mod permission_macros;

// Re-export core entity types used throughout the IAM module
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
    ResourceEnum, // Import ResourceEnum from imphnen_entities
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

// Explicitly export only the imphnen_utils types actually used in IAM
pub use imphnen_utils::{
    response_format::success_response,
    response_format::success_list_response,
    response_format::common_response,
    validator::validate_request,
    csrf_token::generate_oauth_csrf_token,
    csrf_token::validate_oauth_csrf_token,
    csrf_token::validate_csrf_token,
    extract_email::extract_email_async,
    generate_otp::OtpManager,
    errors::AppError,
    response_format::error_response,
    response_format::success_created_response,
    generate_date::get_iso_date,
};
pub use imphnen_libs::AppStatePostgresExt;

// Export the main router functions and types from v1 module
pub use v1::{
    iam_public_routes,
    iam_protected_routes,
    auth_router,
    users_router,
    roles_router,
    permissions_router,
    permissions_guard,
};

// Export permission macros (module not yet implemented)
// pub use permission_macros::*;

// Export IAM-specific types
pub use v1::auth::{
    AuthOtpSchema,
    AuthRepository,
    AuthLoginRequestDto, AuthLoginResponsetDto, AuthRegisterRequestDto,
    AuthResendOtpRequestDto, AuthVerifyEmailRequestDto,
    AuthNewPasswordRequestDto, AuthRefreshTokenRequestDto,
    TokenDto,
};
pub use v1::permissions::{PermissionsRepository, PermissionsSchema};
pub use v1::roles::{RolesRepository, RolesSchema, RolesEnum, RolesDetailQueryDto, RolesRequestCreateDto, RolesRequestUpdateDto, RolesDetailItemDto};
pub use v1::users::{UsersRepository, UsersSchema, UsersDetailItemDto, UsersCreateRequestDto};
