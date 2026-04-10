use super::security::SecurityAddon;
use imphnen_cms::events::infrastructure::http::dto::{
	EventsDetailItemDto, EventsListItemDto,
};
use imphnen_cms::events::infrastructure::http::handlers as events_controller;
use imphnen_cms::qr::campaigns::infrastructure::http::dto::CreateCampaignRequest;
use imphnen_cms::qr::campaigns::infrastructure::http::handlers as qr_campaigns_controller;
use imphnen_cms::qr::users::infrastructure::http::dto::{
	UpdateProfileRequest, UpdateRoleRequest,
};
use imphnen_cms::qr::users::infrastructure::http::handlers as qr_users_controller;
use imphnen_cms::testimonials::infrastructure::http::dto::{
	TestimonialsCreateRequestDto, TestimonialsDetailItemDto, TestimonialsListItemDto,
	TestimonialsUpdateRequestDto,
};
use imphnen_cms::testimonials::infrastructure::http::handlers as testimonials_controller;
use imphnen_dimentorin::mentors::infrastructure::http::dto::{
	IdentityAndVerification, MentorDetailResponseDto, MentorListResponseDto,
	MentorRegisterFromTokenRequestDto, MentorRegisterResponseDto,
	MentorUpdateRequestDto, MentorUserRegisterRequestDto, MentorVerifyRequestDto,
	MentoringLogistics, MentoringRate, ProfessionalProfile,
};
use imphnen_dimentorin::mentors::infrastructure::http::handlers as mentors_controller;
use imphnen_dimentorin::sessions::infrastructure::http::dto::{
	AvailabilitySlotDto, BookSessionRequestDto, BookSessionResponseDto,
	MentorAvailabilityDto, SessionFeedbackRequestDto, SessionFeedbackResponseDto,
	SessionListItemDto, SessionListResponseDto, UpdateSessionStatusRequestDto,
	UpdateSessionStatusResponseDto,
};
use imphnen_dimentorin::sessions::infrastructure::http::handlers as sessions_controller;
use imphnen_entities::{
	MessageResponseDto, ResponseListSuccessDto, ResponseSuccessDto,
};
use imphnen_gacha::gacha_claims::infrastructure::http::dto::{
	GachaClaimCreateRequestDto, GachaClaimDetailDto,
};
use imphnen_gacha::gacha_claims::infrastructure::http::handlers as gacha_claims_controller;
use imphnen_gacha::gacha_credits::infrastructure::http::dto::{
	GachaCreditAddRequestDto, GachaCreditDto,
};
use imphnen_gacha::gacha_credits::infrastructure::http::handlers as gacha_credits_controller;
use imphnen_gacha::gacha_items::infrastructure::http::dto::{
	GachaItemCreateRequestDto, GachaItemDto,
};
use imphnen_gacha::gacha_items::infrastructure::http::handlers as gacha_items_controller;
use imphnen_gacha::gacha_rolls::infrastructure::http::dto::{
	GachaRollCreateRequestDto, GachaRollItemDto,
};
use imphnen_gacha::gacha_rolls::infrastructure::http::handlers as gacha_rolls_controller;
use imphnen_hackathon::admin::domain::entity::{
	AdminSubmissionRow, AdminTeamRow, AdminUserRow, WinnerRow,
};
use imphnen_hackathon::admin::infrastructure::http::dto::{
	SetAdminRequest, SetWinnerRequest,
};
use imphnen_hackathon::admin::infrastructure::http::handlers as hackathon_admin_controller;
use imphnen_hackathon::certificates::infrastructure::http::dto::CertificateResponse;
use imphnen_hackathon::certificates::infrastructure::http::handlers as hackathon_certificates_controller;
use imphnen_hackathon::chat::infrastructure::http::dto::{
	MessageResponse, SendMessageRequest,
};
use imphnen_hackathon::chat::infrastructure::http::handlers as hackathon_chat_controller;
use imphnen_hackathon::invitations::infrastructure::http::dto::{
	CreateInvitationRequest, InvitationResponse, RespondToInvitationRequest,
};
use imphnen_hackathon::invitations::infrastructure::http::handlers as hackathon_invitations_controller;
use imphnen_hackathon::join_requests::infrastructure::http::dto::{
	CreateJoinRequestRequest, JoinRequestResponse, RespondToJoinRequestRequest,
};
use imphnen_hackathon::join_requests::infrastructure::http::handlers as hackathon_join_requests_controller;
use imphnen_hackathon::storage::infrastructure::http::dto::{
	UploadRequest, UploadResponse,
};
use imphnen_hackathon::storage::infrastructure::http::handlers as hackathon_storage_controller;
use imphnen_hackathon::submissions::infrastructure::http::dto::{
	CreateSubmissionRequest, SubmissionResponse, UpdateSubmissionRequest,
};
use imphnen_hackathon::submissions::infrastructure::http::handlers as hackathon_submissions_controller;
use imphnen_hackathon::teams::infrastructure::http::dto::{
	BrowseTeamsQuery, CreateTeamRequest, TeamListResponse, TeamMemberResponse,
	TeamResponse, UpdateTeamRequest, UserInfoResponse,
};
use imphnen_hackathon::teams::infrastructure::http::handlers as hackathon_teams_controller;
use imphnen_hackathon::users::infrastructure::http::dto::{
	UpdateUserRequest, UserResponse,
};
use imphnen_hackathon::users::infrastructure::http::handlers as hackathon_users_controller;
use imphnen_hackathon::winners::infrastructure::http::dto::WinnerResponse;
use imphnen_hackathon::winners::infrastructure::http::handlers as hackathon_winners_controller;
use imphnen_iam::auth::infrastructure::http::dto::{
	AuthLoginRequestDto, AuthLoginResponsetDto, AuthNewPasswordRequestDto,
	AuthRefreshTokenRequestDto, AuthResendOtpRequestDto, AuthVerifyEmailRequestDto,
	TokenDto,
};
use imphnen_iam::auth::infrastructure::http::handlers as auth_controller;
use imphnen_iam::permissions::infrastructure::http::dto::{
	PermissionsCreateRequestDto, PermissionsItemDto,
};
use imphnen_iam::permissions::infrastructure::http::handlers as permissions_controller;
use imphnen_iam::roles::infrastructure::http::dto::{
	RolesCreateRequestDto, RolesDetailItemDto, RolesListItemDto, RolesUpdateRequestDto,
};
use imphnen_iam::roles::infrastructure::http::handlers as roles_controller;
use imphnen_iam::users::infrastructure::http::dto::{
	FileUploadSchema, UsersCreateRequestDto, UsersDetailItemDto, UsersListItemDto,
	UsersUpdateRequestDto,
};
use imphnen_iam::users::infrastructure::http::handlers as users_controller;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        auth_controller::post_login, auth_controller::post_login_mentor,
        auth_controller::post_register, auth_controller::post_verify_email,
        auth_controller::post_resend_otp, auth_controller::post_refresh_token,
        auth_controller::post_forgot_password, auth_controller::post_new_password,
        users_controller::mutation_handlers::post_create_user, users_controller::mutation_handlers::put_update_user,
        users_controller::profile_handlers::put_update_user_me, users_controller::mutation_handlers::patch_user_active_status,
        users_controller::mutation_handlers::delete_user, users_controller::get_handlers::get_user_by_id,
        users_controller::get_handlers::get_user_me, users_controller::get_handlers::get_user_list,
        users_controller::profile_handlers::upload_file,
        roles_controller::get_role_list, roles_controller::get_role_by_id,
        roles_controller::post_create_role, roles_controller::put_update_role,
        roles_controller::delete_role,
        permissions_controller::get_permission_list, permissions_controller::get_permission_by_id,
        permissions_controller::post_create_permission, permissions_controller::put_update_permission,
        permissions_controller::delete_permission,
        gacha_claims_controller::get_gacha_claim_by_id, gacha_claims_controller::post_create_gacha_claim,
        gacha_credits_controller::get_user_credits, gacha_credits_controller::post_add_credits,
        gacha_credits_controller::post_consume_credit,
        gacha_items_controller::get_gacha_item_list, gacha_items_controller::get_gacha_item_by_id,
        gacha_items_controller::post_create_gacha_item, gacha_items_controller::put_update_gacha_item,
        gacha_items_controller::delete_gacha_item,
        gacha_rolls_controller::get_gacha_roll_by_id, gacha_rolls_controller::post_create_gacha_roll,
        gacha_rolls_controller::post_execute_gacha_roll, gacha_rolls_controller::delete_gacha_roll,
        events_controller::get_event_list, events_controller::get_event_by_id,
        events_controller::post_create_event, events_controller::patch_update_event,
        events_controller::delete_event,
        testimonials_controller::get_testimonial_list, testimonials_controller::get_testimonial_by_id,
        testimonials_controller::post_create_testimonial, testimonials_controller::patch_update_testimonial,
        testimonials_controller::delete_testimonial,
        mentors_controller::query_handlers::get_mentor_list, mentors_controller::query_handlers::get_mentor_by_id,
        mentors_controller::mutation_handlers::post_register_mentor, mentors_controller::query_handlers::get_mentor_me,
        mentors_controller::mutation_handlers::put_update_mentor_me, mentors_controller::query_handlers::get_mentor_status,
        mentors_controller::mutation_handlers::put_update_mentor, mentors_controller::mutation_handlers::put_verify_mentor,
        mentors_controller::mutation_handlers::delete_mentor,
        sessions_controller::mutation_handlers::post_book_session, sessions_controller::query_handlers::get_mentor_sessions,
        sessions_controller::query_handlers::get_mentor_availability, sessions_controller::mutation_handlers::put_update_session_status,
        sessions_controller::mutation_handlers::post_submit_feedback, sessions_controller::query_handlers::get_my_sessions,
        hackathon_users_controller::get_me_handler, hackathon_users_controller::update_me_handler,
        hackathon_users_controller::get_user_handler, hackathon_users_controller::get_user_teams_handler,
        hackathon_teams_controller::create_team_handler, hackathon_teams_controller::get_team_handler,
        hackathon_teams_controller::browse_teams_handler, hackathon_teams_controller::get_my_teams_handler,
        hackathon_teams_controller::update_team_handler, hackathon_teams_controller::delete_team_handler,
        hackathon_teams_controller::leave_team_handler, hackathon_teams_controller::remove_member_handler,
        hackathon_invitations_controller::get_my_invitations_handler,
        hackathon_invitations_controller::respond_to_invitation_handler,
        hackathon_invitations_controller::invite_team_member_handler,
        hackathon_join_requests_controller::create_join_request_handler,
        hackathon_join_requests_controller::get_my_join_requests_handler,
        hackathon_join_requests_controller::get_team_join_requests_handler,
        hackathon_join_requests_controller::respond_to_join_request_handler,
        hackathon_submissions_controller::create_submission_handler,
        hackathon_submissions_controller::get_team_submission_handler,
        hackathon_submissions_controller::update_submission_handler,
        hackathon_submissions_controller::submit_project_handler,
        hackathon_submissions_controller::confirm_submission_handler,
        hackathon_submissions_controller::cancel_submission_handler,
        hackathon_certificates_controller::get_certificate_handler,
        hackathon_winners_controller::list_winners_handler,
        hackathon_admin_controller::admin_list_users, hackathon_admin_controller::admin_get_user,
        hackathon_admin_controller::admin_set_admin, hackathon_admin_controller::admin_delete_user,
        hackathon_admin_controller::admin_list_teams, hackathon_admin_controller::admin_delete_team,
        hackathon_admin_controller::admin_list_submissions,
        hackathon_admin_controller::admin_set_winner, hackathon_admin_controller::admin_remove_winner,
        hackathon_admin_controller::admin_list_winners,
        hackathon_chat_controller::get_team_messages_handler,
        hackathon_chat_controller::send_message_handler,
        hackathon_chat_controller::delete_message_handler,
        hackathon_storage_controller::upload_file_handler,
        hackathon_storage_controller::upload_avatar_handler,
        hackathon_storage_controller::upload_team_handler,
        hackathon_storage_controller::upload_submission_handler,
        qr_users_controller::get_me_handler, qr_users_controller::update_me_handler,
        qr_users_controller::list_users_handler, qr_users_controller::update_role_handler,
        qr_users_controller::delete_user_handler,
        qr_campaigns_controller::create_campaign_handler,
        qr_campaigns_controller::list_campaigns_handler,
        qr_campaigns_controller::activate_campaign_handler,
        qr_campaigns_controller::delete_campaign_handler,
        qr_campaigns_controller::process_image_handler,
    ),
    components(schemas(
        MessageResponseDto, AuthLoginRequestDto, AuthLoginResponsetDto,
        AuthVerifyEmailRequestDto, AuthResendOtpRequestDto, AuthNewPasswordRequestDto,
        AuthRefreshTokenRequestDto, ResponseSuccessDto<TokenDto>,
        RolesListItemDto, RolesDetailItemDto, RolesCreateRequestDto, RolesUpdateRequestDto,
        PermissionsCreateRequestDto, PermissionsItemDto,
        UsersDetailItemDto, UsersListItemDto, UsersUpdateRequestDto, UsersCreateRequestDto, FileUploadSchema,
        imphnen_iam::users::infrastructure::http::dto::UsersMeResponseDto,
        imphnen_iam::users::infrastructure::http::dto::HackathonProfileDto,
        imphnen_iam::users::infrastructure::http::dto::QrProfileDto,
        imphnen_iam::users::infrastructure::http::dto::MentorProfileDto,
        imphnen_iam::users::infrastructure::http::dto::SessionProfileDto,
        GachaClaimDetailDto, GachaClaimCreateRequestDto,
        GachaCreditDto, GachaCreditAddRequestDto,
        GachaItemDto, GachaItemCreateRequestDto,
        GachaRollItemDto, GachaRollCreateRequestDto,
        ResponseListSuccessDto<Vec<GachaItemDto>>, ResponseSuccessDto<GachaRollItemDto>,
        ResponseSuccessDto<GachaItemDto>, ResponseSuccessDto<GachaClaimDetailDto>,
        ResponseSuccessDto<AuthLoginResponsetDto>,
        ResponseListSuccessDto<Vec<RolesListItemDto>>, ResponseSuccessDto<RolesDetailItemDto>,
        ResponseListSuccessDto<Vec<UsersListItemDto>>, ResponseSuccessDto<UsersDetailItemDto>,
        ResponseListSuccessDto<Vec<PermissionsItemDto>>, ResponseSuccessDto<PermissionsItemDto>,
        ResponseListSuccessDto<Vec<EventsListItemDto>>, ResponseSuccessDto<EventsDetailItemDto>,
        ResponseListSuccessDto<Vec<TestimonialsListItemDto>>, ResponseSuccessDto<TestimonialsDetailItemDto>,
        TestimonialsCreateRequestDto, TestimonialsUpdateRequestDto,
        MentorUserRegisterRequestDto, MentorRegisterFromTokenRequestDto, MentorRegisterResponseDto,
        MentorListResponseDto, MentorDetailResponseDto, MentorUpdateRequestDto, MentorVerifyRequestDto,
        IdentityAndVerification, ProfessionalProfile, MentoringLogistics, MentoringRate,
        ResponseListSuccessDto<Vec<MentorListResponseDto>>, ResponseSuccessDto<MentorDetailResponseDto>,
        ResponseSuccessDto<MentorRegisterResponseDto>,
        BookSessionRequestDto, BookSessionResponseDto, SessionListResponseDto, SessionListItemDto,
        MentorAvailabilityDto, AvailabilitySlotDto,
        UpdateSessionStatusRequestDto, UpdateSessionStatusResponseDto,
        SessionFeedbackRequestDto, SessionFeedbackResponseDto,
        ResponseSuccessDto<BookSessionResponseDto>, ResponseSuccessDto<SessionListResponseDto>,
        ResponseSuccessDto<MentorAvailabilityDto>, ResponseSuccessDto<UpdateSessionStatusResponseDto>,
        ResponseSuccessDto<SessionFeedbackResponseDto>,
        UserResponse, UpdateUserRequest,
        TeamResponse, CreateTeamRequest, UpdateTeamRequest, BrowseTeamsQuery,
        TeamListResponse, UserInfoResponse, TeamMemberResponse,
        InvitationResponse, RespondToInvitationRequest, CreateInvitationRequest,
        JoinRequestResponse, CreateJoinRequestRequest, RespondToJoinRequestRequest,
        SubmissionResponse, CreateSubmissionRequest, UpdateSubmissionRequest,
        CertificateResponse, WinnerResponse,
        SetAdminRequest, SetWinnerRequest,
        AdminUserRow, AdminTeamRow, AdminSubmissionRow, WinnerRow,
        MessageResponse, SendMessageRequest,
        UploadRequest, UploadResponse,
        UpdateProfileRequest, UpdateRoleRequest, CreateCampaignRequest,
    )),
    info(
        title = "IMPHNEN Backend Service",
        description = "IMPHNEN Backend Service for Provide Gacha, Dimentorin and Backoffice Web App",
        version = "0.1.0",
        contact(name = "Maulana Sodiqin", url = ""),
        license(name = "MIT", url = "https://opensource.org/licenses/MIT")
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Authentication", description = "IAM — Auth endpoints (/v1/iam/auth)"),
        (name = "Users", description = "IAM — User management (/v1/iam/users)"),
        (name = "Roles", description = "IAM — Role management (/v1/iam/roles)"),
        (name = "Permissions", description = "IAM — Permission management (/v1/iam/permissions)"),
        (name = "Events", description = "Landing CMS — Event management (/v1/landing/cms/events)"),
        (name = "Testimonials", description = "Landing CMS — Testimonial management (/v1/landing/cms/testimonials)"),
        (name = "Mentors", description = "Dimentorin — Mentor management (/v1/dimentorin/mentors)"),
        (name = "Mentors - Admin", description = "Dimentorin — Mentor admin endpoints (/v1/dimentorin/mentors)"),
        (name = "sessions", description = "Dimentorin — Session management (/v1/dimentorin/sessions)"),
        (name = "Gacha", description = "Gacha system (/v1/gacha)"),
        (name = "Hackathon - Users", description = "Hackathon — User profile (/v1/hackathon/users)"),
        (name = "Hackathon - Teams", description = "Hackathon — Team management (/v1/hackathon/teams)"),
        (name = "Hackathon - Invitations", description = "Hackathon — Team invitations (/v1/hackathon/invitations)"),
        (name = "Hackathon - Join Requests", description = "Hackathon — Join requests (/v1/hackathon/join-requests)"),
        (name = "Hackathon - Submissions", description = "Hackathon — Project submissions (/v1/hackathon/submissions)"),
        (name = "Hackathon - Certificates", description = "Hackathon — Certificates (/v1/hackathon/certificates)"),
        (name = "Hackathon - Winners", description = "Hackathon — Winners (/v1/hackathon/winners)"),
        (name = "Hackathon - Admin", description = "Hackathon — Admin management (/v1/hackathon/admin)"),
        (name = "Hackathon - Chat", description = "Hackathon — Team chat (/v1/hackathon/chat)"),
        (name = "Hackathon - Storage", description = "Hackathon — File uploads (/v1/hackathon/upload)"),
        (name = "QR - Users", description = "QR — User management (/v1/qr/users)"),
        (name = "QR - Campaigns", description = "QR — Campaign management (/v1/qr/campaigns)"),
    )
)]
pub struct ApiDoc;

pub fn docs_router() -> utoipa::openapi::OpenApi {
	ApiDoc::openapi()
}
