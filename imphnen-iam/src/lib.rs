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
};

// Explicitly export only the imphnen_libs types actually used in IAM
pub use imphnen_libs::{
    AppState,
    ResourceEnum,
    decode_access_token,
    decode_refresh_token,
    encode_access_token,
    encode_refresh_token,
    encode_reset_password_token,
    hash_password,
    send_email,
    verify_password,
    Env,
    SurrealWsClient,
    SurrealMemClient,
    UserLookupService,
    AuthRepositoryTrait,
    jsonwebtoken::Claims,
};

// Explicitly export only the imphnen_utils types actually used in IAM
pub use imphnen_utils::{
    make_thing,
    make_thing_from_enum,
    get_id,
    get_iso_date,
    extract_id,
    build_multi_thing_condition,
    execute_safe_update_query,
    DetailQueryBuilder,
    QueryListBuilder,
    success_response,
    success_list_response,
    common_response,
    validate_request,
    generate_oauth_csrf_token,
    validate_oauth_csrf_token,
    validate_csrf_token,
    extract_email_token_async,
    OtpManager,
};

// Export the main router functions and types from v1 module
pub use v1::{
    iam_public_routes,
    iam_protected_routes,
    auth_router,
    users_router,
    roles_router,
    permissions_router,
    teams_router,
    permissions_guard,
};

// Export permission macros
pub use permission_macros::{check_permissions, check_authenticated};

// Export IAM-specific types
pub use v1::auth::{
    AuthRepository, AuthOtpSchema,
    AuthLoginRequestDto, AuthLoginResponsetDto, AuthRegisterRequestDto,
    AuthResendOtpRequestDto, AuthVerifyEmailRequestDto,
    AuthNewPasswordRequestDto, AuthRefreshTokenRequestDto,
    TokenDto, UserCacheSchema,
};
pub use v1::permissions::{PermissionsRepository, PermissionsSchema};
pub use v1::roles::{RolesRepository, RolesSchema, RolesEnum, RolesDetailQueryDto, RolesRequestCreateDto, RolesRequestUpdateDto, RolesDetailItemDto};
pub use v1::teams::{
    TeamsRepository, TeamsSchema, TeamsCreateRequestDto, TeamsUpdateRequestDto,
    TeamInviteRequestDto, TeamMemberDto, AdminTeamsListItemDto,
    AdminTeamsDetailItemDto, TeamsDetailItemDto, TeamsListItemDto,
    TeamAcceptInvitationRequestDto, TeamsSearchQueryDto, PublicTeamsListItemDto,
    PublicTeamsDetailItemDto, TeamsDetailQueryDto, TeamsListQueryDto,
    TeamMembersSchema, TeamInvitationsSchema, TeamMembersQueryDto,
    TeamInvitationsQueryDto, MemberTeamsDetailItemDto,
    AddTeamMemberRequestDto, UpdateMemberRoleRequestDto,
    TeamInvitationListDto, MyInvitationDto
};
pub use v1::users::{UsersRepository, UsersSchema, UsersDetailItemDto, UsersCreateRequestDto};
