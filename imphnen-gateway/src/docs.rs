use imphnen_cms::{
	events_controller,
	events_dto::{EventsDetailItemDto, EventsListItemDto},
	testimonials_controller,
	testimonials_dto::{
		TestimonialsCreateRequestDto, TestimonialsDetailItemDto,
		TestimonialsListItemDto, TestimonialsUpdateRequestDto,
	},
};
use imphnen_dimentorin::v1::mentors::{
	mentors_controller,
	mentors_dto::{
		IdentityAndVerification, MentorDetailResponseDto, MentorListResponseDto,
		MentorRegisterFromTokenRequestDto, MentorRegisterResponseDto,
		MentorUpdateRequestDto, MentorUserRegisterRequestDto, MentorVerifyRequestDto,
		MentoringLogistics, MentoringRate, ProfessionalProfile,
	},
};
use imphnen_gacha::{
	GachaClaimItemDto, GachaClaimRequestDto, GachaItemDto, GachaItemRequestDto,
	GachaRollItemDto, GachaRollRequestDto, gacha_claims, gacha_items, gacha_rolls,
};
use imphnen_iam::{
	AuthLoginRequestDto, AuthLoginResponsetDto, AuthNewPasswordRequestDto,
	AuthRefreshTokenRequestDto, AuthResendOtpRequestDto, AuthVerifyEmailRequestDto,
	MessageResponseDto, MetaRequestDto, MetaResponseDto, PermissionsItemDto,
	PermissionsRequestDto, ResponseListSuccessDto, ResponseSuccessDto,
	RolesDetailItemDto, RolesListItemDto, RolesRequestCreateDto,
	RolesRequestUpdateDto, TokenDto, UsersCreateRequestDto, UsersDetailItemDto,
	UsersListItemDto, UsersUpdateRequestDto, auth, permissions, roles, users,
};
use imphnen_iam::users::users_controller::FileUploadSchema;
use utoipa::{
	Modify, OpenApi,
	openapi::security::{Http, HttpAuthScheme, SecurityScheme},
};

#[derive(OpenApi)]
#[openapi(
    paths(
     auth::auth_controller::post_login,
     auth::auth_controller::post_login_mentor,
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
     users::users_controller::upload_file,
     roles::roles_controller::get_role_list,
     roles::roles_controller::get_role_by_id,
     roles::roles_controller::post_create_role,
     roles::roles_controller::put_update_role,
     roles::roles_controller::delete_role,
     permissions::permissions_controller::get_permission_list,
     permissions::permissions_controller::get_permission_by_id,
     permissions::permissions_controller::post_create_permission,
     permissions::permissions_controller::put_update_permission,
     permissions::permissions_controller::delete_permission,
     gacha_claims::get_detail_gacha_claim,
     gacha_claims::post_create_gacha_claim,
     gacha_items::get_gacha_item_list,
     gacha_items::get_gacha_item_by_id,
     gacha_items::post_create_gacha_item,
     gacha_items::put_update_gacha_item,
     gacha_items::delete_gacha_item,
     gacha_rolls::get_detail_gacha_roll,
     gacha_rolls::post_create_gacha_roll,
     gacha_rolls::post_execute_gacha_roll,
     events_controller::get_event_list,
     events_controller::get_event_by_id,
     events_controller::post_create_event,
     events_controller::patch_update_event,
     events_controller::delete_event,
     testimonials_controller::get_testimonial_list,
     testimonials_controller::get_testimonial_by_id,
     testimonials_controller::post_create_testimonial,
     testimonials_controller::patch_update_testimonial,
     testimonials_controller::delete_testimonial,
     mentors_controller::get_mentor_list,
     mentors_controller::get_mentor_by_id,
     mentors_controller::post_register_mentor,
     mentors_controller::get_mentor_me,
     mentors_controller::put_update_mentor_me,
     mentors_controller::get_mentor_status,
     mentors_controller::put_update_mentor,
     mentors_controller::put_verify_mentor,
     mentors_controller::delete_mentor,
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
           FileUploadSchema,
           GachaClaimItemDto,
           GachaClaimRequestDto,
           GachaItemDto,
           GachaItemRequestDto,
           GachaRollItemDto,
           GachaRollRequestDto,
           ResponseListSuccessDto<Vec<GachaItemDto>>,
           ResponseSuccessDto<GachaRollItemDto>,
           ResponseSuccessDto<GachaItemDto>,
           ResponseSuccessDto<GachaClaimItemDto>,
           ResponseSuccessDto<AuthLoginResponsetDto>,
           ResponseListSuccessDto<Vec<RolesListItemDto>>,
           ResponseSuccessDto<RolesDetailItemDto>,
           ResponseListSuccessDto<Vec<UsersListItemDto>>,
           ResponseSuccessDto<UsersDetailItemDto>,
           ResponseListSuccessDto<Vec<PermissionsItemDto>>,
           ResponseSuccessDto<PermissionsItemDto>,
           ResponseListSuccessDto<Vec<EventsListItemDto>>,
           ResponseSuccessDto<EventsDetailItemDto>,
           ResponseListSuccessDto<Vec<TestimonialsListItemDto>>,
           ResponseSuccessDto<TestimonialsDetailItemDto>,
           TestimonialsCreateRequestDto,
           TestimonialsUpdateRequestDto,
           MentorUserRegisterRequestDto,
           MentorRegisterFromTokenRequestDto,
           MentorRegisterResponseDto,
           MentorListResponseDto,
           MentorDetailResponseDto,
           MentorUpdateRequestDto,
           MentorVerifyRequestDto,
           IdentityAndVerification,
           ProfessionalProfile,
           MentoringLogistics,
           MentoringRate,
           ResponseListSuccessDto<Vec<MentorListResponseDto>>,
           ResponseSuccessDto<MentorDetailResponseDto>,
           ResponseSuccessDto<MentorRegisterResponseDto>,
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
        (name = "Users", description = "User Management Endpoints"),
        (name = "Roles", description = "Role Management Endpoints"),
        (name = "Permissions", description = "Permission Management Endpoints"),
        (name = "Events", description = "Event Management Endpoints"),
        (name = "Testimonials", description = "Testimonial Management Endpoints"),
        (name = "Mentors", description = "Mentor Management Endpoints"),
        (name = "Mentors - Admin", description = "Mentor Admin Management Endpoints (Admin Access Required)"),
        (name = "Gacha", description = "Gacha System Endpoints"),
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
