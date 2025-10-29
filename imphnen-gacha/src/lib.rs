pub mod v1;

// Re-export core entity types used across the gacha system
pub use imphnen_entities::{
    CountResult,
    Error,
    ExperienceDto,
    EducationDto,
    MessageResponseDto,
    MetaRequestDto,
    MetaResponseDto,
    PermissionsEnum,
    PermissionsItemDto,
    PermissionsQueryDto,
    ResponseListSuccessDto,
    ResponseSuccessDto,
    UsersDetailQueryDto,
};

// Explicitly import only what we need from libs and utils to avoid pollution
pub use imphnen_libs::{
    AppState,
    MinioService,
};

pub use imphnen_utils::{
    bind_filter,
    csrf_token,
    extract_email,
    generate_date,
    generate_otp,
    get_id,
    logger,
    make_thing,
    query_builder,
    query_list,
    response_format,
    serde_helpers,
    validator,
};

// Re-export public v1 API
pub use v1::gacha_router;
