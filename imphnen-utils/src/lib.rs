//! # imphnen-utils
//!
//! A collection of utility functions and types for the imphnen project.
//!
//! This crate provides various utilities including OTP generation with expiration and hashing,
//! CSRF token management, email extraction from tokens, query building for SurrealDB,
//! and standardized response formatting.

pub mod bind_filter;
pub mod csrf_token;
pub mod extract_email;
pub mod generate_date;
pub mod generate_otp;
pub mod get_id;
pub mod logger;
pub mod make_thing;
pub mod query_builder;
pub mod query_list;
pub mod response_format;
pub mod serde_helpers;
pub mod validator;

// Internal module re-exports
pub use bind_filter::bind_filter_value;
pub use csrf_token::{generate_csrf_token, generate_oauth_csrf_token, validate_csrf_token, validate_oauth_csrf_token};
pub use extract_email::{extract_email, extract_email_async, extract_email_token, extract_email_token_async};
pub use generate_date::get_iso_date;
pub use generate_otp::OtpManager;
pub use get_id::{extract_id, get_id};
pub use logger::init_logger;
pub use make_thing::{make_thing, make_thing_from_enum, make_thing_str};
pub use query_builder::{
    build_multi_thing_condition,
    build_thing_condition,
    execute_safe_count_query,
    execute_safe_update_query,
    DetailQueryBuilder,
    ListQueryBuilder,
};
pub use query_list::QueryListBuilder;
pub use response_format::{common_response, success_created_response, success_list_response, success_response};
pub use serde_helpers::{
    deserialize_datetime,
    option_thing_or_string,
    serialize_datetime,
    serialize_option_thing,
    serialize_thing,
    string_or_empty_string,
    thing_or_string,
};
pub use validator::validate_request;

// External crate re-exports
pub use imphnen_libs::{
    AppState,
    Claims,
    CountResult,
    EducationDto,
    ENV,
    Env,
    Error,
    ExperienceDto,
    FileMetadata,
    FileType,
    MessageResponseDto,
    MetaRequestDto,
    MetaResponseDto,
    MinioConfig,
    MinioService,
    PermissionsEnum,
    PermissionsItemDto,
    PermissionsQueryDto,
    ResourceEnum,
    ResponseListSuccessDto,
    ResponseSuccessDto,
    SurrealMemClient,
    SurrealWsClient,
    UploadRequest,
    UploadResult,
    UserLookupService,
    UsersDetailQueryDto,
    AuthRepositoryTrait,
    axum_init,
    create_minio_service_from_config,
    decode_access_token,
    decode_base64_file,
    decode_refresh_token,
    encode_access_token,
    encode_refresh_token,
    encode_reset_password_token,
    extract_content_type_from_data_url,
    generate_jwt,
    hash_password,
    send_email,
    surrealdb_init_mem,
    surrealdb_init_ws,
    verify_password,
};
