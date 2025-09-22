use crate::v1::mentors::{
	MentorDetailQueryDto, MentorDetailResponseDto, MentorListResponseDto,
	MentorRegisterResponseDto, MentorSchema, MentorUpdateRequestDto,
	MentorUserRegisterRequestDto, MentorVerifyRequestDto, MentorsRepository,
};
use axum::http::StatusCode;
use axum::response::Response;
use imphnen_entities::{
	AppState, MetaRequestDto, ResponseListSuccessDto, ResponseSuccessDto,
};
use imphnen_iam::{
	AuthRepository, RolesEnum, RolesRepository, UsersRepository, UsersSchema,
};
use imphnen_libs::ResourceEnum;
use imphnen_utils::{
	common_response, success_list_response, success_response, validate_request,
};
use surrealdb::Uuid;
use surrealdb::sql::Thing;
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
		let auth_repo = AuthRepository::new(state);

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

			user_schema.fullname = dto.fullname.clone();
			user_schema.phone_number = dto.phone_number.clone();
			// Update personal data from identity_and_verification
			user_schema.legal_name = Some(dto.identity_and_verification.legal_name.clone());
			user_schema.gender = dto.identity_and_verification.gender.clone();
			user_schema.domicile = dto.identity_and_verification.domicile.clone();
			user_schema.phone_for_verification = Some(dto.identity_and_verification.phone_for_verification.clone());
			// Update personal data from professional_profile
			user_schema.bio = Some(dto.professional_profile.bio.clone());
			user_schema.last_education = dto.professional_profile.last_education.clone();
			user_schema.linkedin_url = dto.professional_profile.linkedin_url.clone();
			user_schema.github_url = dto.professional_profile.github_url.clone();
			user_schema.cv_url = dto.professional_profile.cv_url.clone();
			user_schema.portfolio_url = dto.professional_profile.portfolio_url.clone();
			user_schema.updated_at = imphnen_utils::get_iso_date();

			let hashed_password = match imphnen_utils::hash_password(&dto.password) {
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
			user_schema.password = hashed_password;

			let mentor_role = match role_repo
				.query_role_by_name(RolesEnum::Mentor.to_string())
				.await
			{
				Ok(role) => role,
				Err(_e) => {
					return common_response(StatusCode::BAD_REQUEST, "Mentor Role Not Found");
				}
			};
			user_schema.role =
				imphnen_utils::make_thing_from_enum(ResourceEnum::Roles, &mentor_role.id);
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

			let hashed_password = match imphnen_utils::hash_password(&dto.password) {
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

			let new_user_schema = UsersSchema {
				id: imphnen_utils::make_thing_from_enum(
					ResourceEnum::Users,
					&Uuid::new_v4().to_string(),
				),
				email: dto.email,
				fullname: dto.fullname,
				password: hashed_password,
				phone_number: dto.phone_number,
				// Store personal data from identity_and_verification in user
				legal_name: Some(dto.identity_and_verification.legal_name.clone()),
				gender: dto.identity_and_verification.gender.clone(),
				domicile: dto.identity_and_verification.domicile.clone(),
				phone_for_verification: Some(dto.identity_and_verification.phone_for_verification.clone()),
				// Store personal data from professional_profile in user
				bio: Some(dto.professional_profile.bio.clone()),
				last_education: dto.professional_profile.last_education.clone(),
				linkedin_url: dto.professional_profile.linkedin_url.clone(),
				github_url: dto.professional_profile.github_url.clone(),
				cv_url: dto.professional_profile.cv_url.clone(),
				portfolio_url: dto.professional_profile.portfolio_url.clone(),
				created_at: imphnen_utils::get_iso_date(),
				updated_at: imphnen_utils::get_iso_date(),
				role: imphnen_utils::make_thing_from_enum(
					ResourceEnum::Roles,
					&mentor_role.id,
				),
				is_active: false,
				..Default::default()
			};

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

		let otp = imphnen_utils::generate_otp::OtpManager::generate_otp();

		match auth_repo
			.query_store_otp(final_user_email.clone(), otp)
			.await
		{
			Ok(_) => {
				let message = format!("your otp code is {otp}");
				if let Err(_err) =
					imphnen_utils::send_email(&final_user_email, "OTP Verification", &message)
				{
					error!("Failed to send OTP email to {}: {}", final_user_email, _err);
					return common_response(
						StatusCode::INTERNAL_SERVER_ERROR,
						&_err.to_string(),
					);
				}
			}
			Err(_err) => {
				error!("Failed to store OTP for {}: {}", final_user_email, _err);
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					&_err.to_string(),
				);
			}
		}

		let mentor_schema = MentorSchema::create(
			dto.professional_profile,
			dto.mentoring_logistics,
			user_id.to_raw(),
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
					let mentor_dto = MentorDetailQueryDto::from(mentor_with_user);
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
		let thing_id = Thing::from((ResourceEnum::Mentors.to_string().as_str(), id));
		
		match mentor_repo.query_mentor_by_id(&thing_id, false).await {
			Ok(mentor) => {
				// Get user data separately
				let user_result = user_repo.query_user_by_id(&mentor.user_id).await;
				match user_result {
					Ok(user) => {
						// Combine mentor and user data
						let dto = MentorDetailResponseDto {
							id: mentor.id.to_raw(),
							user_id: mentor.user_id.to_raw(),
							// Personal data from user
							fullname: Some(user.fullname),
							email: Some(user.email),
							legal_name: user.legal_name,
							gender: user.gender,
							domicile: user.domicile,
							phone_for_verification: user.phone_for_verification,
							bio: user.bio,
							last_education: user.last_education,
							linkedin_url: user.linkedin_url,
							github_url: user.github_url,
							cv_url: user.cv_url,
							portfolio_url: user.portfolio_url,
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
		let thing_id = Thing::from((ResourceEnum::Mentors.to_string().as_str(), id));
		let existing_mentor = match repo.query_mentor_by_id(&thing_id, false).await {
			Ok(mentor) => mentor,
			Err(_e) => return common_response(StatusCode::NOT_FOUND, &_e.to_string()),
		};

		let mut schema = MentorSchema::from(MentorDetailQueryDto::from(existing_mentor));
		schema = schema.update(dto);

		match repo.query_update_mentor(schema).await {
			Ok(_) => {
				let updated_mentor =
					repo.query_mentor_by_id(&thing_id, false).await.unwrap();
				let response_dto =
					MentorDetailResponseDto::from(MentorDetailQueryDto::from(updated_mentor));
				success_response(ResponseSuccessDto { data: response_dto })
			}
			Err(_e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &_e.to_string()),
		}
	}

	pub async fn delete_mentor(state: &AppState, id: &str) -> Response {
		let repo = MentorsRepository::new(state);
		match repo.query_delete_mentor(id.to_string()).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(_e) => common_response(StatusCode::NOT_FOUND, &_e.to_string()),
		}
	}

	pub async fn get_mentor_me(state: &AppState, email: &str) -> Response {
		let repo = MentorsRepository::new(state);
		match repo.query_mentor_by_email(email.to_string(), false).await {
			Ok(mentor) => {
				let dto = MentorDetailResponseDto::from(MentorDetailQueryDto::from(mentor));
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

		let mut schema = MentorSchema::from(MentorDetailQueryDto::from(existing_mentor));
		schema = schema.update(dto);

		match repo.query_update_mentor(schema).await {
			Ok(_) => {
				let updated_mentor = repo
					.query_mentor_by_email(email.to_string(), false)
					.await
					.unwrap();
				let response_dto =
					MentorDetailResponseDto::from(MentorDetailQueryDto::from(updated_mentor));
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
		let thing_id = Thing::from((ResourceEnum::Mentors.to_string().as_str(), id));
		let existing_mentor = match repo.query_mentor_by_id(&thing_id, false).await {
			Ok(mentor) => mentor,
			Err(_e) => return common_response(StatusCode::NOT_FOUND, &_e.to_string()),
		};

		let mut schema = MentorSchema::from(MentorDetailQueryDto::from(existing_mentor));
		schema = schema.update_status(dto.status);

		match repo.query_update_mentor(schema).await {
			Ok(_) => {
				let updated_mentor =
					repo.query_mentor_by_id(&thing_id, false).await.unwrap();
				let response_dto =
					MentorDetailResponseDto::from(MentorDetailQueryDto::from(updated_mentor));
				success_response(ResponseSuccessDto { data: response_dto })
			}
			Err(_e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &_e.to_string()),
		}
	}
}
