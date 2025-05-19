use imphnen_iam::{
auth, permissions, roles, users, AuthLoginRequestDto, AuthLoginResponsetDto, AuthNewPasswordRequestDto, AuthRefreshTokenRequestDto, AuthResendOtpRequestDto, AuthVerifyEmailRequestDto, MessageResponseDto, MetaRequestDto, MetaResponseDto, PermissionsItemDto, PermissionsRequestDto, ResponseListSuccessDto, ResponseSuccessDto, RolesDetailItemDto, RolesListItemDto, RolesRequestCreateDto, RolesRequestUpdateDto, TokenDto, UsersCreateRequestDto, UsersDetailItemDto, UsersListItemDto, UsersUpdateRequestDto
};
use utoipa::{
	openapi::security::{Http, HttpAuthScheme, SecurityScheme},
	Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    paths(
     auth::auth_controller::post_login,
     auth::auth_controller::post_register,
     auth::auth_controller::post_verify_email,
     auth::auth_controller::post_resend_otp,
     auth::auth_controller::post_refresh_token,
     auth::auth_controller::post_forgot_password,
     auth::auth_controller::post_new_password,
     users::users_controller::post_create_user,
     users::users_controller::put_update_user,
     users::users_controller::put_update_user_me,
     users::users_controller::patch_user_active_status,
     users::users_controller::delete_user,
     users::users_controller::get_user_by_id,
     users::users_controller::get_user_me,
     users::users_controller::get_user_list,
     roles::roles_controller::get_role_list,
     roles::roles_controller::get_role_by_id,
     roles::roles_controller::post_create_role,
     roles::roles_controller::put_update_role,
     roles::roles_controller::delete_role,
     permissions::permissions_controller::get_permission_list,
     permissions::permissions_controller::get_permission_by_id,
     permissions::permissions_controller::post_create_permission,
     permissions::permissions_controller::put_update_permission,
     permissions::permissions_controller::delete_permission
    ),
    components(
        schemas(
           MetaRequestDto,
           MetaResponseDto,
           MessageResponseDto,
           AuthLoginRequestDto,
           AuthLoginResponsetDto,
           AuthVerifyEmailRequestDto,
           AuthResendOtpRequestDto,
           AuthNewPasswordRequestDto,
           AuthRefreshTokenRequestDto,
           ResponseSuccessDto<TokenDto>,
           RolesListItemDto,
           RolesRequestCreateDto, 
           RolesRequestUpdateDto,
           PermissionsRequestDto,
           PermissionsItemDto,
           UsersDetailItemDto,
           UsersListItemDto,
           UsersUpdateRequestDto,
           UsersCreateRequestDto,
           ResponseSuccessDto<AuthLoginResponsetDto>,
           ResponseListSuccessDto<Vec<RolesListItemDto>>,
           ResponseSuccessDto<RolesDetailItemDto>,
           ResponseListSuccessDto<Vec<UsersListItemDto>>,
           ResponseSuccessDto<UsersDetailItemDto>,
           ResponseListSuccessDto<Vec<PermissionsItemDto>>,
           ResponseSuccessDto<PermissionsItemDto>
        )
    ),
    info(
        title = "IMPHNEN Backend Service",
        description = "IMPHNEN Backend Service for Provide Gacha, Dimentorin and Backoffice Web App",
        version = "0.1.0",
        contact(
            name = "Maulana Sodiqin",
            url = ""
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Authentication", description = "List of Authentication Endpoints"),
    )
)]

pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
	fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
		if let Some(components) = openapi.components.as_mut() {
			components.add_security_scheme(
				"Bearer",
				SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
			);
		}
	}
}

pub fn docs_router() -> utoipa::openapi::OpenApi {
	ApiDoc::openapi()
}
