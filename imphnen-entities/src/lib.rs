pub mod common_dto;
pub mod error_dto;
pub mod users;
pub mod permissions;
pub mod audit_log;
pub mod seaorm;

// Re-export error type at root level for convenience
pub use error_dto::error::Error;

// Explicit common_dto exports
pub use common_dto::CountResult;
pub use common_dto::ErrorDto;
pub use common_dto::MessageResponseDto;
pub use common_dto::MetaRequestDto;
pub use common_dto::MetaResponseDto;
pub use common_dto::ResponseListSuccessDto;
pub use common_dto::ResponseSuccessDto;

// Explicit users exports
pub use users::EducationDto;
pub use users::ExperienceDto;
pub use users::RolesDetailItemDto;
pub use users::RolesDetailQueryDto;
pub use users::UsersDetailQueryDto;

// Explicit permissions exports
pub use permissions::PermissionsEnum;
pub use permissions::PermissionsItemDto;
pub use permissions::PermissionsQueryDto;
pub use seaorm::common::enums::ResourceEnum;

// Explicit audit_log exports
pub use audit_log::AuditLogSchema;

// SeaORM entity exports
pub use seaorm::*;
