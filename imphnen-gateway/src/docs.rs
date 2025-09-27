use imphnen_cms::v1::landing::events::events_controller;
use imphnen_cms::v1::landing::events::events_dto::{EventsDetailItemDto, EventsListItemDto};
use imphnen_cms::v1::landing::testimonials::testimonials_controller;
use imphnen_cms::v1::landing::testimonials::testimonials_dto::{
	TestimonialsCreateRequestDto, TestimonialsDetailItemDto,
	TestimonialsListItemDto, TestimonialsUpdateRequestDto,
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
use imphnen_gacha::v1::gacha_claims::{gacha_claims_controller, GachaClaimItemDto, GachaClaimRequestDto};
use imphnen_gacha::v1::gacha_items::{gacha_items_controller, GachaItemDto};
use imphnen_gacha::v1::gacha_items::gacha_items_dto::GachaItemRequestDto;
use imphnen_gacha::v1::gacha_rolls::{gacha_rolls_controller, GachaRollItemDto};
use imphnen_gacha::v1::gacha_rolls::gacha_rolls_dto::GachaRollRequestDto;
use imphnen_hackathon::v1::hackathon::{
    hackathon_controller,
    hackathon_dto::{
        HackathonCreateRequestDto, HackathonDto, HackathonEventCreateRequestDto, HackathonEventDto,
        HackathonEventUpdateRequestDto, HackathonSubmissionCreateRequestDto,
        HackathonSubmissionDto, HackathonSubmissionUpdateRequestDto, HackathonTimelineCreateRequestDto,
        HackathonTimelineDto, HackathonTimelineUpdateRequestDto, HackathonUpdateRequestDto,
    },
};
use imphnen_entities::{PermissionsItemDto, RolesDetailItemDto};
use imphnen_entities::{MessageResponseDto, MetaRequestDto, MetaResponseDto, ResponseListSuccessDto, ResponseSuccessDto};
use imphnen_iam::v1::auth::auth_dto::{AuthLoginRequestDto, AuthLoginResponsetDto, AuthNewPasswordRequestDto, AuthRefreshTokenRequestDto, AuthResendOtpRequestDto, AuthVerifyEmailRequestDto, TokenDto};
use imphnen_iam::v1::permissions::permissions_dto::PermissionsRequestDto;
use imphnen_iam::v1::roles::RolesListItemDto;
use imphnen_iam::v1::roles::roles_dto::{RolesRequestCreateDto, RolesRequestUpdateDto};
use imphnen_iam::v1::users::UsersDetailItemDto;
use imphnen_iam::v1::users::users_dto::{UsersCreateRequestDto, UsersListItemDto, UsersUpdateRequestDto};
use imphnen_iam::v1::teams::teams_dto::{TeamsCreateRequestDto, TeamsUpdateRequestDto, TeamInviteRequestDto, TeamAcceptInvitationRequestDto, TeamsDetailItemDto, TeamsListItemDto, TeamMemberDto, TeamInvitationDto, TeamsSearchQueryDto};
use imphnen_iam::v1::{auth, permissions, roles, users, teams};
use imphnen_iam::v1::users::users_controller::FileUploadSchema;
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
           teams::teams_controller::get_team_list,
           teams::teams_controller::get_team_by_id,
           teams::teams_controller::post_create_team,
           teams::teams_controller::put_update_team,
           teams::teams_controller::delete_team,
           teams::teams_controller::post_invite_team_members,
           teams::teams_controller::post_accept_invitation,
           teams::teams_controller::get_public_team_search,
           teams::teams_controller::get_team_members,
           teams::teams_controller::post_leave_team,
           gacha_claims_controller::get_detail_gacha_claim,
           gacha_claims_controller::post_create_gacha_claim,
           gacha_items_controller::get_gacha_item_list,
           gacha_items_controller::get_gacha_item_by_id,
           gacha_items_controller::post_create_gacha_item,
           gacha_items_controller::put_update_gacha_item,
           gacha_items_controller::delete_gacha_item,
           gacha_rolls_controller::get_detail_gacha_roll,
           gacha_rolls_controller::post_create_gacha_roll,
           gacha_rolls_controller::post_execute_gacha_roll,
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
     hackathon_controller::create_hackathon,
     hackathon_controller::get_hackathon,
     hackathon_controller::list_hackathons,
     hackathon_controller::update_hackathon,
     hackathon_controller::delete_hackathon,
     hackathon_controller::create_hackathon_event,
     hackathon_controller::list_hackathon_events,
     hackathon_controller::update_hackathon_event,
     hackathon_controller::delete_hackathon_event,
     hackathon_controller::create_hackathon_timeline,
     hackathon_controller::list_hackathon_timeline,
     hackathon_controller::update_hackathon_timeline,
     hackathon_controller::delete_hackathon_timeline,
     hackathon_controller::create_hackathon_submission,
     hackathon_controller::list_hackathon_submissions,
     hackathon_controller::update_hackathon_submission,
     hackathon_controller::submit_hackathon_submission,
     hackathon_controller::delete_hackathon_submission,
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
                       TeamsCreateRequestDto,
                       TeamsUpdateRequestDto,
                       TeamInviteRequestDto,
                       TeamAcceptInvitationRequestDto,
                       TeamsDetailItemDto,
                       TeamsListItemDto,
                       TeamMemberDto,
                       TeamInvitationDto,
                       TeamsSearchQueryDto,
                       ResponseListSuccessDto<Vec<TeamsListItemDto>>,
                       ResponseSuccessDto<TeamsDetailItemDto>,
                       HackathonCreateRequestDto,
                       HackathonDto,
                       HackathonEventCreateRequestDto,
                       HackathonEventDto,
                       HackathonEventUpdateRequestDto,
                       HackathonSubmissionCreateRequestDto,
                       HackathonSubmissionDto,
                       HackathonSubmissionUpdateRequestDto,
                       HackathonTimelineCreateRequestDto,
                       HackathonTimelineDto,
                       HackathonTimelineUpdateRequestDto,
                       HackathonUpdateRequestDto,
                       ResponseListSuccessDto<Vec<HackathonDto>>,
                       ResponseSuccessDto<HackathonDto>,
                       ResponseListSuccessDto<Vec<HackathonEventDto>>,
                       ResponseSuccessDto<HackathonEventDto>,
                       ResponseListSuccessDto<Vec<HackathonSubmissionDto>>,
                       ResponseSuccessDto<HackathonSubmissionDto>,
                       ResponseListSuccessDto<Vec<HackathonTimelineDto>>,
                       ResponseSuccessDto<HackathonTimelineDto>,
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
        (name = "Hackathons", description = "Hackathon Management Endpoints"),
        (name = "Hackathon Events", description = "Hackathon Event Management Endpoints"),
        (name = "Hackathon Timeline", description = "Hackathon Timeline Management Endpoints"),
        (name = "Hackathon Submissions", description = "Hackathon Submission Management Endpoints"),
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
	}
}

pub fn docs_router() -> utoipa::openapi::OpenApi {
	ApiDoc::openapi()
}
