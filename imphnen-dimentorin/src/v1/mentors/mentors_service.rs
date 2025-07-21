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

		let user_email = dto.email.clone();
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

			let mut user_schema = UsersSchema::from(user_detail_query_dto.clone());

			user_schema.fullname = dto.fullname.clone();
			user_schema.phone_number = dto.phone_number.clone();
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
				imphnen_utils::make_thing(&ResourceEnum::Roles.to_string(), &mentor_role.id);
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
				id: imphnen_utils::make_thing(
					&ResourceEnum::Users.to_string(),
					&Uuid::new_v4().to_string(),
				),
				email: dto.email.clone(),
				fullname: dto.fullname.clone(),
				password: hashed_password,
				phone_number: dto.phone_number.clone(),
				created_at: imphnen_utils::get_iso_date(),
				updated_at: imphnen_utils::get_iso_date(),
				role: imphnen_utils::make_thing(
					&ResourceEnum::Roles.to_string(),
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
			dto.identity_and_verification,
			dto.professional_profile,
			dto.mentoring_logistics,
			user_id.to_raw(),
			final_user_email.clone(),
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
		match repo.query_mentor_list(meta).await {
			Ok(result) => {
				let data: Vec<MentorListResponseDto> = result
					.data
					.into_iter()
					.map(MentorDetailQueryDto::from)
					.map(MentorListResponseDto::from)
					.collect();
				success_list_response(ResponseListSuccessDto {
					data,
					meta: result.meta,
				})
			}
			Err(_e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &_e.to_string()),
		}
	}

	pub async fn get_mentor_by_id(state: &AppState, id: &str) -> Response {
		let repo = MentorsRepository::new(state);
		let thing_id = Thing::from((ResourceEnum::Mentors.to_string().as_str(), id));
		match repo.query_mentor_by_id(&thing_id, false).await {
			Ok(mentor) => {
				let dto = MentorDetailResponseDto::from(MentorDetailQueryDto::from(mentor));
				success_response(ResponseSuccessDto { data: dto })
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
