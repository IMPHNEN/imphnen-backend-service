use imphnen_cms::events::infrastructure::http::handlers as events_controller;
use imphnen_cms::events::infrastructure::http::dto::{EventsDetailItemDto, EventsListItemDto};
use imphnen_cms::testimonials::infrastructure::http::handlers as testimonials_controller;
use imphnen_cms::testimonials::infrastructure::http::dto::{
	TestimonialsCreateRequestDto, TestimonialsDetailItemDto,
	TestimonialsListItemDto, TestimonialsUpdateRequestDto,
};
use imphnen_dimentorin::mentors::infrastructure::http::handlers as mentors_controller;
use imphnen_dimentorin::mentors::infrastructure::http::dto::{
	IdentityAndVerification, MentorDetailResponseDto, MentorListResponseDto,
	MentorRegisterFromTokenRequestDto, MentorRegisterResponseDto,
	MentorUpdateRequestDto, MentorUserRegisterRequestDto, MentorVerifyRequestDto,
	MentoringLogistics, MentoringRate, ProfessionalProfile,
};
use imphnen_dimentorin::sessions::infrastructure::http::handlers as sessions_controller;
use imphnen_dimentorin::sessions::infrastructure::http::dto::{
	BookSessionRequestDto, BookSessionResponseDto, MentorAvailabilityDto,
	SessionFeedbackRequestDto, SessionFeedbackResponseDto, SessionListItemDto,
	SessionListResponseDto, UpdateSessionStatusRequestDto, UpdateSessionStatusResponseDto,
	AvailabilitySlotDto,
};
use imphnen_gacha::gacha_claims::infrastructure::http::handlers as gacha_claims_controller;
use imphnen_gacha::gacha_claims::infrastructure::http::dto::{GachaClaimDetailDto, GachaClaimCreateRequestDto};
use imphnen_gacha::gacha_items::infrastructure::http::handlers as gacha_items_controller;
use imphnen_gacha::gacha_items::infrastructure::http::dto::{GachaItemDto, GachaItemCreateRequestDto};
use imphnen_gacha::gacha_rolls::infrastructure::http::handlers as gacha_rolls_controller;
use imphnen_gacha::gacha_rolls::infrastructure::http::dto::{GachaRollItemDto, GachaRollCreateRequestDto};
use imphnen_entities::{MessageResponseDto, ResponseListSuccessDto, ResponseSuccessDto};
use imphnen_iam::auth::infrastructure::http::dto::{AuthLoginRequestDto, AuthLoginResponsetDto, AuthNewPasswordRequestDto, AuthRefreshTokenRequestDto, AuthResendOtpRequestDto, AuthVerifyEmailRequestDto, TokenDto};
use imphnen_iam::permissions::infrastructure::http::handlers as permissions_controller;
use imphnen_iam::permissions::infrastructure::http::dto::{PermissionsCreateRequestDto, PermissionsItemDto};
use imphnen_iam::roles::infrastructure::http::handlers as roles_controller;
use imphnen_iam::roles::infrastructure::http::dto::{RolesDetailItemDto, RolesListItemDto, RolesCreateRequestDto, RolesUpdateRequestDto};
use imphnen_iam::users::infrastructure::http::handlers as users_controller;
use imphnen_iam::users::infrastructure::http::dto::{UsersDetailItemDto, UsersCreateRequestDto, UsersListItemDto, UsersUpdateRequestDto, FileUploadSchema};
use imphnen_iam::auth::infrastructure::http::handlers as auth_controller;
use utoipa::{
    Modify, OpenApi,
    openapi::security::{Http, HttpAuthScheme, SecurityScheme, SecurityRequirement},
};

#[derive(OpenApi)]
#[openapi(
    paths(
     auth_controller::post_login,
     auth_controller::post_login_mentor,
     auth_controller::post_register,
     auth_controller::post_verify_email,
     auth_controller::post_resend_otp,
     auth_controller::post_refresh_token,
     auth_controller::post_forgot_password,
     auth_controller::post_new_password,
     users_controller::post_create_user,
     users_controller::put_update_user,
     users_controller::put_update_user_me,
     users_controller::patch_user_active_status,
     users_controller::delete_user,
     users_controller::get_user_by_id,
     users_controller::get_user_me,
     users_controller::get_user_list,
     users_controller::upload_file,
     roles_controller::get_role_list,
     roles_controller::get_role_by_id,
     roles_controller::post_create_role,
     roles_controller::put_update_role,
     roles_controller::delete_role,
     permissions_controller::get_permission_list,
     permissions_controller::get_permission_by_id,
     permissions_controller::post_create_permission,
     permissions_controller::put_update_permission,
     permissions_controller::delete_permission,
           gacha_claims_controller::get_gacha_claim_by_id,
           gacha_claims_controller::post_create_gacha_claim,
           gacha_items_controller::get_gacha_item_list,
           gacha_items_controller::get_gacha_item_by_id,
           gacha_items_controller::post_create_gacha_item,
           gacha_items_controller::put_update_gacha_item,
           gacha_items_controller::delete_gacha_item,
           gacha_rolls_controller::get_gacha_roll_by_id,
           gacha_rolls_controller::post_create_gacha_roll,
           gacha_rolls_controller::post_execute_gacha_roll,
           gacha_rolls_controller::delete_gacha_roll,
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
     sessions_controller::post_book_session,
     sessions_controller::get_mentor_sessions,
     sessions_controller::get_mentor_availability,
     sessions_controller::put_update_session_status,
     sessions_controller::post_submit_feedback,
     sessions_controller::get_my_sessions,
    ),
    components(
        schemas(
           MessageResponseDto,
           AuthLoginRequestDto,
           AuthLoginResponsetDto,
           AuthVerifyEmailRequestDto,
           AuthResendOtpRequestDto,
           AuthNewPasswordRequestDto,
           AuthRefreshTokenRequestDto,
           ResponseSuccessDto<TokenDto>,
           RolesListItemDto,
           RolesDetailItemDto,
           RolesCreateRequestDto,
           RolesUpdateRequestDto,
           PermissionsCreateRequestDto,
           PermissionsItemDto,
           UsersDetailItemDto,
           UsersListItemDto,
           UsersUpdateRequestDto,
           UsersCreateRequestDto,
           FileUploadSchema,
           GachaClaimDetailDto,
           GachaClaimCreateRequestDto,
           GachaItemDto,
           GachaItemCreateRequestDto,
           GachaRollItemDto,
           GachaRollCreateRequestDto,
           ResponseListSuccessDto<Vec<GachaItemDto>>,
           ResponseSuccessDto<GachaRollItemDto>,
           ResponseSuccessDto<GachaItemDto>,
           ResponseSuccessDto<GachaClaimDetailDto>,
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
           BookSessionRequestDto,
           BookSessionResponseDto,
           SessionListResponseDto,
           SessionListItemDto,
           MentorAvailabilityDto,
           AvailabilitySlotDto,
           UpdateSessionStatusRequestDto,
           UpdateSessionStatusResponseDto,
           SessionFeedbackRequestDto,
           SessionFeedbackResponseDto,
           ResponseSuccessDto<BookSessionResponseDto>,
           ResponseSuccessDto<SessionListResponseDto>,
           ResponseSuccessDto<MentorAvailabilityDto>,
           ResponseSuccessDto<UpdateSessionStatusResponseDto>,
           ResponseSuccessDto<SessionFeedbackResponseDto>,
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
        (name = "sessions", description = "Mentoring Sessions Management API"),
        (name = "Gacha", description = "Gacha System Endpoints"),
    )
)]
    pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
	fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
		if let Some(components) = openapi.components.as_mut() {
			components.add_security_scheme(
				"Bearer",
				SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
			);
		}

        // Walk all paths and add a Bearer security requirement to any operation
        // that declares 401 or 403 responses. This helps ensure protected
        // endpoints are shown with the Bearer lock in the generated docs
        // without having to annotate every controller manually.
        let paths = &mut openapi.paths;
        for (_path, path_item) in paths.paths.iter_mut() {
                // helper to process each possible operation on the path
                let process_op = |op: &mut Option<utoipa::openapi::path::Operation>| {
                    if let Some(operation) = op.as_mut() {
                        let mut has_auth_response = false;
                        let responses = &operation.responses.responses;
                        for status in responses.keys() {
                            if status == "401" || status == "403" {
                                has_auth_response = true;
                                break;
                            }
                        }
                        if has_auth_response {
                            // assign security requirement for Bearer if not already present
                            if operation.security.is_none() {
                                operation.security = Some(vec![SecurityRequirement::new::<&str, Vec<&str>, &str>("Bearer", vec![])]);
                            }
                        }
                    }
                };

                process_op(&mut path_item.get);
                process_op(&mut path_item.post);
                process_op(&mut path_item.put);
                process_op(&mut path_item.patch);
                process_op(&mut path_item.delete);
                process_op(&mut path_item.options);
                process_op(&mut path_item.head);
                process_op(&mut path_item.trace);
        }
    }
}

pub fn docs_router() -> utoipa::openapi::OpenApi {
	ApiDoc::openapi()
}
