use axum::{
	Router,
	routing::{delete, get, post, put},
};

pub mod mentors_controller;
pub mod mentors_dto;
pub mod mentors_repository;
pub mod mentors_schema;
pub mod mentors_service;

// Explicitly export only public controller functions and key types
pub use mentors_controller::{
    post_register_mentor,
    get_mentor_list,
    get_mentor_by_id,
    put_update_mentor,
    delete_mentor,
    put_verify_mentor,
    get_mentor_me,
    put_update_mentor_me,
    put_update_mentor_no_id,
    get_mentor_status,
};

// Export key DTO types used across the API
pub use mentors_dto::{
    MentorListResponseDto,
    MentorDetailResponseDto,
    MentorRegisterResponseDto,
    MentorUpdateRequestDto,
    MentorUserRegisterRequestDto,
    MentorVerifyRequestDto,
    MentorDetailQueryDto,
    ProfessionalProfile,
    MentoringLogistics,
    MentoringRate,
    IdentityAndVerification,
    MentorInsertDto,
};

// Export service and repository for internal use
pub use mentors_service::MentorsService;
pub use mentors_repository::MentorsRepository;

// Export schema types for database interactions
pub use mentors_schema::MentorSchema;

pub fn mentors_router() -> Router {
	Router::new()
		.route("/", get(get_mentor_list))
		.route("/create", post(post_register_mentor))
		.route("/me", get(get_mentor_me))
		.route("/me/update", put(put_update_mentor_me))
		.route("/me/status", get(get_mentor_status))
		.route("/detail/{id}", get(get_mentor_by_id))
		.route("/update/{id}", put(put_update_mentor))
		.route("/update", put(put_update_mentor_no_id))
		.route("/delete/{id}", delete(delete_mentor))
		.route("/verify/{id}", put(put_verify_mentor))
}
