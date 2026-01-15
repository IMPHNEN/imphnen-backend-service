use crate::v1::mentors::{
	MentorDetailResponseDto, MentorListResponseDto,
	MentorRegisterResponseDto, MentorSchema, MentorUpdateRequestDto,
	MentorUserRegisterRequestDto, MentorVerifyRequestDto, MentorsRepository,
};
use axum::http::StatusCode;
use axum::response::Response;
use imphnen_entities::{
	MetaRequestDto, ResponseListSuccessDto, ResponseSuccessDto,
};
use imphnen_libs::AppState;
use imphnen_iam::{
	v1::auth::AuthRepository,
	RolesEnum, RolesRepository, UsersRepository, UsersSchema,
};
use imphnen_libs::argon::hash_password;
use imphnen_utils::{
	common_response, success_list_response, success_response, validator::validate_request,
};
use uuid::Uuid;
use tracing::error;

pub struct MentorsService;

impl MentorsService {
	pub async fn register_mentor(
		state: &AppState,
		dto: MentorUserRegisterRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&dto) {
			return common_response(status, &message);
		}

		let user_repo = UsersRepository::new(state);
		let mentor_repo = MentorsRepository::new(state);
		let role_repo = RolesRepository::new(state);
		let _auth_repo = AuthRepository::new(state);

		let user_email = &dto.email;
		let mut _user_to_update: Option<UsersSchema> = None;

		let existing_user_result =
			user_repo.query_user_by_email(user_email.clone()).await;

		let user_id;
		let final_user_email = user_email.clone();

		if let Ok(user_detail_query_dto) = existing_user_result {
			if mentor_repo
				.query_mentor_by_email(user_email.clone(), false)
				.await
				.is_ok()
			{
				return common_response(
					StatusCode::CONFLICT,
					"Mentor profile already exists for this user",
				);
			}

			let mut user_schema = UsersSchema::from(user_detail_query_dto);

			user_schema.fullname = Some(dto.fullname.clone());
			// Update profile_extension fields
			let mut profile_ext = user_schema.profile_extension.clone().unwrap_or_default();
			profile_ext.phone_number = dto.phone_number.clone();
			profile_ext.phone_for_verification = dto.identity_and_verification.phone_for_verification.clone();
			profile_ext.gender = dto.identity_and_verification.gender.clone();
			profile_ext.domicile = dto.identity_and_verification.domicile.clone();
			profile_ext.bio = Some(dto.professional_profile.bio.clone());
			profile_ext.last_education = dto.professional_profile.last_education.clone();
			profile_ext.linkedin_url = dto.professional_profile.linkedin_url.clone();
			profile_ext.github_url = dto.professional_profile.github_url.clone();
			profile_ext.cv_url = dto.professional_profile.cv_url.clone();
			profile_ext.portfolio_url = dto.professional_profile.portfolio_url.clone();
			user_schema.profile_extension = Some(profile_ext);
			user_schema.updated_at = imphnen_utils::get_iso_date();

			let hashed_password = match hash_password(&dto.password) {
				Ok(hash) => hash,
				Err(_e) => {
					error!(
						"Failed to hash password during update for {}: {}",
						final_user_email, _e
					);
					return common_response(
						StatusCode::INTERNAL_SERVER_ERROR,
						"Failed to hash password",
					);
				}
			};
			user_schema.password = Some(hashed_password);

			let mentor_role = match role_repo
				.query_role_by_name(RolesEnum::Mentor.to_string())
				.await
			{
				Ok(role) => role,
				Err(_e) => {
					return common_response(StatusCode::BAD_REQUEST, "Mentor Role Not Found");
				}
			};
			user_schema.mentor_id = Some(imphnen_utils::make_thing_from_enum("Roles", &mentor_role.id));
			user_schema.is_active = false;

			if let Err(_err) = user_repo.query_update_user(user_schema.clone()).await {
				error!(
					"Failed to update existing user {} to mentor role: {}",
					final_user_email, _err
				);
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					&_err.to_string(),
				);
			}
			user_id = user_schema.id.clone();
		} else {
			let mentor_role = match role_repo
				.query_role_by_name(RolesEnum::Mentor.to_string())
				.await
			{
				Ok(role) => role,
				Err(_e) => {
					return common_response(StatusCode::BAD_REQUEST, "Mentor Role Not Found");
				}
			};

			let hashed_password = match hash_password(&dto.password) {
				Ok(hash) => hash,
				Err(_e) => {
					error!(
						"Failed to hash password for new user {}: {}",
						final_user_email, _e
					);
					return common_response(
						StatusCode::INTERNAL_SERVER_ERROR,
						"Failed to hash password",
					);
				}
			};

			let mut new_user_schema = UsersSchema {
				id: imphnen_utils::make_thing_from_enum(
					"Users",
					&Uuid::new_v4().to_string(),
				),
				email: Some(dto.email),
				fullname: Some(dto.fullname),
				password: Some(hashed_password),
				// Set phone number in profile extension instead
				// Store personal data from identity_and_verification in user
				legal_name: Some(dto.identity_and_verification.legal_name.clone()),
				// Use profile_extension for these fields
				// Store personal data from professional_profile in user
				created_at: imphnen_utils::get_iso_date(),
				updated_at: imphnen_utils::get_iso_date(),
				mentor_id: Some(imphnen_utils::make_thing_from_enum(
					"Roles",
					&mentor_role.id,
				)),
				is_active: false,
				..Default::default()
			};

			// populate profile_extension
			let mut profile_ext = new_user_schema.profile_extension.clone().unwrap_or_default();
			profile_ext.phone_number = dto.phone_number.clone();
			profile_ext.phone_for_verification = dto.identity_and_verification.phone_for_verification.clone();
			profile_ext.gender = dto.identity_and_verification.gender.clone();
			profile_ext.domicile = dto.identity_and_verification.domicile.clone();
			profile_ext.bio = Some(dto.professional_profile.bio.clone());
			profile_ext.last_education = dto.professional_profile.last_education.clone();
			profile_ext.linkedin_url = dto.professional_profile.linkedin_url.clone();
			profile_ext.github_url = dto.professional_profile.github_url.clone();
			profile_ext.cv_url = dto.professional_profile.cv_url.clone();
			profile_ext.portfolio_url = dto.professional_profile.portfolio_url.clone();
			new_user_schema.profile_extension = Some(profile_ext);
			user_id = new_user_schema.id.clone();

			match user_repo.query_create_user(new_user_schema).await {
				Ok(_) => {}
				Err(_err) => {
					error!("Failed to create new user {}: {}", final_user_email, _err);
					return common_response(
						StatusCode::INTERNAL_SERVER_ERROR,
						&_err.to_string(),
					);
				}
			}
		}

		// Skip OTP for now - implement later if needed

		let mentor_schema = MentorSchema::create(
			dto.professional_profile,
			dto.mentoring_logistics,
			user_id.clone(),
		);

		match mentor_repo.query_create_mentor(mentor_schema.clone()).await {
			Ok(mentor_profile_id) => {
				let user_after_mentor_creation_dto = user_repo
					.query_user_by_email(final_user_email.clone())
					.await
					.unwrap();
				let mut user_after_mentor_creation_schema =
					UsersSchema::from(user_after_mentor_creation_dto);
				user_after_mentor_creation_schema = user_after_mentor_creation_schema
					.update_mentor_id(Some(mentor_profile_id));
				if let Err(_e) = user_repo
					.query_update_user(user_after_mentor_creation_schema)
					.await
				{
					error!(
						"Failed to update user's mentor_id for {}: {}",
						final_user_email, _e
					);
					return common_response(
						StatusCode::INTERNAL_SERVER_ERROR,
						&_e.to_string(),
					);
				}

				let response_dto = MentorRegisterResponseDto::from(mentor_schema);
				success_response(ResponseSuccessDto { data: response_dto })
			}
			Err(_e) => {
				error!(
					"Failed to create mentor profile for {}: {}",
					final_user_email, _e
				);
				common_response(StatusCode::INTERNAL_SERVER_ERROR, &_e.to_string())
			}
		}
	}

	pub async fn get_mentor_list(state: &AppState, meta: MetaRequestDto) -> Response {
		let repo = MentorsRepository::new(state);
		let user_repo = UsersRepository::new(state);
		
		match repo.query_mentor_list(meta).await {
			Ok(result) => {
				let mut mentor_list_data: Vec<MentorListResponseDto> = Vec::new();
				
				for mentor_with_user in result.data {
					let mentor_dto = mentor_with_user;
					let mut list_item = MentorListResponseDto::from(mentor_dto.clone());
					
					// Get user data to populate personal fields
					if let Ok(user) = user_repo.query_user_by_id(&mentor_dto.user_id).await {
						list_item.fullname = Some(user.fullname);
						list_item.email = Some(user.email);
					}
					
					mentor_list_data.push(list_item);
				}
				
				success_list_response(ResponseListSuccessDto {
					data: mentor_list_data,
					meta: result.meta,
				})
			}
			Err(_e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &_e.to_string()),
		}
	}

	pub async fn get_mentor_by_id(state: &AppState, id: &str) -> Response {
		let mentor_repo = MentorsRepository::new(state);
		let user_repo = UsersRepository::new(state);
        let thing_id = imphnen_utils::make_thing_from_enum("Mentors", id);		match mentor_repo.query_mentor_by_id(&thing_id, false).await {
			Ok(mentor) => {
				// Get user data separately
				let user_result = user_repo.query_user_by_id(&mentor.user_id).await;
				match user_result {
					Ok(user) => {
						// Combine mentor and user data
						let dto = MentorDetailResponseDto {
							id: mentor.id.clone(),
							user_id: mentor.user_id.clone(),
							// Personal data from user
							fullname: Some(user.fullname),
							email: Some(user.email),
							legal_name: user.legal_name,
									gender: user.profile_extension.as_ref().and_then(|ext| ext.gender.clone()),
									domicile: user.profile_extension.as_ref().and_then(|ext| ext.domicile.clone()),
									phone_for_verification: user.profile_extension.as_ref().and_then(|ext| ext.phone_for_verification.clone()),
									bio: user.profile_extension.as_ref().and_then(|ext| ext.bio.clone()),
									last_education: user.profile_extension.as_ref().and_then(|ext| ext.last_education.clone()),
									linkedin_url: user.profile_extension.as_ref().and_then(|ext| ext.linkedin_url.clone()),
									github_url: user.profile_extension.as_ref().and_then(|ext| ext.github_url.clone()),
									cv_url: user.profile_extension.as_ref().and_then(|ext| ext.cv_url.clone()),
									portfolio_url: user.profile_extension.as_ref().and_then(|ext| ext.portfolio_url.clone()),
							// Professional data from mentor
							industries: mentor.industries,
							expertise: mentor.expertise,
							languages: mentor.languages,
							current_company: mentor.current_company,
							current_role: mentor.current_role,
							years_of_experience: mentor.years_of_experience,
							topics_of_interest: mentor.topics_of_interest,
							preferred_mentee_level: mentor.preferred_mentee_level,
							preferred_mentoring_formats: mentor.preferred_mentoring_formats,
							availability_commitment: mentor.availability_commitment,
							mentoring_rate: mentor.mentoring_rate,
							status: mentor.status,
							created_at: mentor.created_at,
							updated_at: mentor.updated_at,
						};
						success_response(ResponseSuccessDto { data: dto })
					}
					Err(_e) => {
						error!("Failed to get user data for mentor {}: {}", id, _e);
						common_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to get mentor user data")
					}
				}
			}
			Err(_e) => common_response(StatusCode::NOT_FOUND, &_e.to_string()),
		}
	}

	pub async fn update_mentor(
		state: &AppState,
		id: &str,
		dto: MentorUpdateRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&dto) {
			return common_response(status, &message);
		}
		let repo = MentorsRepository::new(state);
		let mentor_uuid = Uuid::parse_str(id).map_err(|_| {
					common_response(
						StatusCode::BAD_REQUEST,
						"Invalid mentor ID format"
					)
				}).unwrap();
				let existing_mentor = match repo.query_mentor_by_id(&mentor_uuid.to_string(), false).await {
			Ok(mentor) => mentor,
			Err(_e) => return common_response(StatusCode::NOT_FOUND, &_e.to_string()),
		};

		let mut schema = MentorSchema::from(existing_mentor);
		schema = schema.update(dto);

		match repo.query_update_mentor(schema).await {
			Ok(_) => {
				let updated_mentor =
					repo.query_mentor_by_id(id, false).await.unwrap();
				let response_dto =
					MentorDetailResponseDto::from(updated_mentor);
				success_response(ResponseSuccessDto { data: response_dto })
			}
			Err(_e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &_e.to_string()),
		}
	}

	pub async fn delete_mentor(state: &AppState, id: &str) -> Response {
		let repo = MentorsRepository::new(state);
		match repo.query_delete_mentor(id).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(_e) => common_response(StatusCode::NOT_FOUND, &_e.to_string()),
		}
	}

	pub async fn get_mentor_me(state: &AppState, email: &str) -> Response {
		let repo = MentorsRepository::new(state);
		match repo.query_mentor_by_email(email.to_string(), false).await {
			Ok(mentor) => {
				let dto = MentorDetailResponseDto::from(mentor);
				success_response(ResponseSuccessDto { data: dto })
			}
			Err(_e) => common_response(
				StatusCode::FORBIDDEN,
				"Mentor profile not found for current user",
			),
		}
	}

	pub async fn update_mentor_me(
		state: &AppState,
		email: &str,
		dto: MentorUpdateRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&dto) {
			return common_response(status, &message);
		}
		let repo = MentorsRepository::new(state);
		let existing_mentor =
			match repo.query_mentor_by_email(email.to_string(), false).await {
				Ok(mentor) => mentor,
				Err(_e) => return common_response(StatusCode::FORBIDDEN, &_e.to_string()),
			};

		let mut schema = MentorSchema::from(existing_mentor);
		schema = schema.update(dto);

		match repo.query_update_mentor(schema).await {
			Ok(_) => {
				let updated_mentor = repo
					.query_mentor_by_email(email.to_string(), false)
					.await
					.unwrap();
				let response_dto =
					MentorDetailResponseDto::from(updated_mentor);
				success_response(ResponseSuccessDto { data: response_dto })
			}
			Err(_e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &_e.to_string()),
		}
	}

	pub async fn get_mentor_status(state: &AppState, email: &str) -> Response {
		let repo = MentorsRepository::new(state);
		match repo.query_mentor_by_email(email.to_string(), false).await {
			Ok(mentor) => common_response(StatusCode::OK, &mentor.status),
			Err(_e) => common_response(
				StatusCode::FORBIDDEN,
				"No mentor application found for current user",
			),
		}
	}

	pub async fn verify_mentor(
		state: &AppState,
		id: &str,
		dto: MentorVerifyRequestDto,
	) -> Response {
		let repo = MentorsRepository::new(state);
		let mentor_uuid = Uuid::parse_str(id).map_err(|_| {
					common_response(
						StatusCode::BAD_REQUEST,
						"Invalid mentor ID format"
					)
				}).unwrap();
				let existing_mentor = match repo.query_mentor_by_id(&mentor_uuid.to_string(), false).await {
			Ok(mentor) => mentor,
			Err(_e) => return common_response(StatusCode::NOT_FOUND, &_e.to_string()),
		};

		let mut schema = MentorSchema::from(existing_mentor);
		schema = schema.update_status(dto.status);

		match repo.query_update_mentor(schema).await {
			Ok(_) => {
				let updated_mentor =
					repo.query_mentor_by_id(&mentor_uuid.to_string(), false).await.unwrap();
				let response_dto =
					MentorDetailResponseDto::from(updated_mentor);
				success_response(ResponseSuccessDto { data: response_dto })
			}
			Err(_e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &_e.to_string()),
		}
	}
}
