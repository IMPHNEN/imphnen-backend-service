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
    csrf_token,
    extract_email,
    generate_date,
    generate_otp,
    logger,
    make_thing,
    response_format,
    validator,
};

// Re-export public v1 API
pub use v1::gacha_router;