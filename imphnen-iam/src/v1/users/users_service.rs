use super::{
	UsersActiveInactiveRequestDto, UsersCreateRequestDto,
	UsersSetNewPasswordRequestDto, UsersUpdateRequestDto,
};
use crate::{
	AppState, MetaRequestDto, ResponseListSuccessDto, UsersRepository, UsersSchema,
};
use crate::{
	ResponseSuccessDto, common_response, extract_email, success_list_response,
	success_response, validate_request,
};
use axum::http::HeaderMap;
use axum::{http::StatusCode, response::Response};
use imphnen_libs::{ResourceEnum, hash_password, verify_password, surrealdb_init_ws, surrealdb_init_mem};
use imphnen_utils::make_thing;
use uuid::Uuid;
use anyhow::Result;
use async_trait::async_trait;
use tracing::info;
use crate::v1::users::users_dto::{UsersDetailItemDto as UserDto, UsersCreateRequestDto as CreateUserDto};

#[async_trait]
pub trait UsersServiceTrait: Send + Sync + 'static {
    async fn get_user_list(state: &AppState, meta: MetaRequestDto) -> Response;
    async fn get_user_by_id(state: &AppState, id: String) -> Response;
    async fn get_user_me(headers: HeaderMap, state: &AppState) -> Response;
    async fn create_user(state: &AppState, new_user: UsersCreateRequestDto) -> Response;
    async fn update_user(state: &AppState, id: String, user: UsersUpdateRequestDto) -> Response;
    async fn update_user_me(headers: HeaderMap, state: &AppState, user: UsersUpdateRequestDto) -> Response;
    async fn set_user_active_status(state: &AppState, id: String, payload: UsersActiveInactiveRequestDto) -> Response;
    async fn update_user_password(state: &AppState, email: String, payload: UsersSetNewPasswordRequestDto) -> Response;
    async fn get_user_by_mentor_id(state: &AppState, mentor_id: String) -> Response;
    async fn delete_user(state: &AppState, id: String) -> Response;

    async fn get_user_by_email(&self, email: &str) -> Result<Option<UserDto>>;
    async fn create_user_by_dto(&self, new_user: CreateUserDto) -> Result<UserDto>;
    async fn update_user_avatar(&self, email: &str, avatar_url: Option<String>) -> Result<()>;
}

#[derive(Clone)]
pub struct UsersService;

#[async_trait]
impl UsersServiceTrait for UsersService {
	async fn get_user_list(state: &AppState, meta: MetaRequestDto) -> Response {
		let repo = UsersRepository::new(state);
		match repo.query_user_list(meta).await {
			Ok(data) => {
				let response = ResponseListSuccessDto {
					data: data.data,
					meta: data.meta,
				};
				success_list_response(response)
			}
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	async fn get_user_by_id(state: &AppState, id: String) -> Response {
		if Uuid::parse_str(&id).is_err() {
            return common_response(StatusCode::BAD_REQUEST, "Invalid User ID format");
        }
		let repo = UsersRepository::new(state);
		let thing_id = make_thing(&ResourceEnum::Users.to_string(), &id);
		match repo.query_user_by_id(&thing_id).await {
			Ok(user) if !user.is_deleted => success_response(ResponseSuccessDto {
				data: UserDto::from(&user), // Corrected to use UserDto::from by reference
			}),
			Ok(_) => common_response(StatusCode::NOT_FOUND, "User not found"),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	async fn get_user_me(headers: HeaderMap, state: &AppState) -> Response {
		let repo = UsersRepository::new(state);
		let email = match extract_email(&headers) {
			Some(email) => email,
			None => return common_response(StatusCode::UNAUTHORIZED, "Invalid token"),
		};
		match repo.query_user_by_email(email).await {
			Ok(user) if !user.is_deleted => success_response(ResponseSuccessDto {
				data: UserDto::from(&user), // Corrected to use UserDto::from by reference
			}),
			Ok(_) => common_response(StatusCode::NOT_FOUND, "User not found"),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	async fn create_user(
		state: &AppState,
		new_user: UsersCreateRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&new_user) {
			return common_response(status, &message);
		}
		let repo = UsersRepository::new(state);
		if repo
			.query_user_by_email(new_user.email.clone())
			.await
			.is_ok()
		{
			return common_response(StatusCode::BAD_REQUEST, "User already exists");
		}
		match repo.query_create_user(UsersSchema::create(new_user)).await {
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(err) => {
				common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
			}
		}
	}

	async fn update_user(
		state: &AppState,
		id: String,
		user: UsersUpdateRequestDto,
	) -> Response {
		if Uuid::parse_str(&id).is_err() {
            return common_response(StatusCode::BAD_REQUEST, "Invalid User ID format");
        }
		let repo = UsersRepository::new(state);
		if let Err((status, message)) = validate_request(&user) {
			return common_response(status, &message);
		}
		let updated_user = UsersSchema::update(user, id);
		match repo.query_update_user(updated_user).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	async fn update_user_me(
		headers: HeaderMap,
		state: &AppState,
		user: UsersUpdateRequestDto,
	) -> Response {
		let repo = UsersRepository::new(state);
		let email = match extract_email(&headers) {
			Some(email) => email,
			None => return common_response(StatusCode::UNAUTHORIZED, "Unauthorized"),
		};
		let user_data = match repo.query_user_by_email(email.clone()).await {
			Ok(user) => user,
			Err(_) => return common_response(StatusCode::NOT_FOUND, "User not found"),
		};
		if let Err((status, message)) = validate_request(&user) {
			return common_response(status, &message);
		}
		let updated_user = UsersSchema::update(user, user_data.id.id.to_raw());
		match repo.query_update_user(updated_user).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	async fn set_user_active_status(
		state: &AppState,
		id: String,
		payload: UsersActiveInactiveRequestDto,
	) -> Response {
		if Uuid::parse_str(&id).is_err() {
            return common_response(StatusCode::BAD_REQUEST, "Invalid User ID format");
        }
		let repo = UsersRepository::new(state);
		let thing_id = make_thing(&ResourceEnum::Users.to_string(), &id);
		match repo.query_user_by_id(&thing_id).await {
			Ok(user) if !user.is_deleted => {
				let patch = UsersSchema {
					id: user.id.clone(),
					is_active: payload.is_active,
					..UsersSchema::from(user)
				};
				match repo.query_update_user(patch).await {
					Ok(msg) => common_response(StatusCode::OK, &msg),
					Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
				}
			}
			Ok(_) => common_response(StatusCode::NOT_FOUND, "User not found"),
			Err(err) => common_response(StatusCode::BAD_REQUEST, &err.to_string()),
		}
	}

	async fn update_user_password(
		state: &AppState,
		email: String,
		payload: UsersSetNewPasswordRequestDto,
	) -> Response {
		let repo = UsersRepository::new(state);
		let user = match repo.query_user_by_email(email.clone()).await {
			Ok(user) if !user.is_deleted => user,
			_ => return common_response(StatusCode::NOT_FOUND, "User not found"),
		};
		let verify_result = match verify_password(&payload.old_password, &user.password)
		{
			Ok(result) => result,
			Err(_) => {
				return common_response(
					StatusCode::BAD_REQUEST,
					"Old password is incorrect",
				);
			}
		};
		if !verify_result {
			return common_response(StatusCode::BAD_REQUEST, "Old password is incorrect");
		}
		let new_password = match hash_password(&payload.password) {
			Ok(pw) => pw,
			Err(_) => {
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					"Failed to hash password",
				);
			}
		};
		let patch = UsersSchema {
			id: user.id.clone(),
			password: new_password,
			..Default::default()
		};
		match repo.query_update_user(patch).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	async fn get_user_by_mentor_id(
		state: &AppState,
		mentor_id: String,
	) -> Response {
		let repo = UsersRepository::new(state);
		let thing_id = make_thing(&ResourceEnum::Mentors.to_string(), &mentor_id);
		match repo.query_user_by_id(&thing_id).await {
			Ok(user) if !user.is_deleted => success_response(ResponseSuccessDto {
				data: UserDto::from(&user), // Corrected to use UserDto::from by reference
			}),
			Ok(_) => common_response(StatusCode::NOT_FOUND, "User not found"),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	async fn delete_user(state: &AppState, id: String) -> Response {
		if Uuid::parse_str(&id).is_err() {
            return common_response(StatusCode::BAD_REQUEST, "Invalid User ID format");
        }
		let repo = UsersRepository::new(state);
		let thing_id = make_thing(&ResourceEnum::Users.to_string(), &id);
		if repo.query_user_by_id(&thing_id).await.is_err() {
			return common_response(StatusCode::BAD_REQUEST, "User not found");
		}
		match repo.query_delete_user(id).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

    #[allow(unused_variables)]
    async fn get_user_by_email(&self, email: &str) -> Result<Option<UserDto>> {
        let surrealdb_ws = surrealdb_init_ws().await
            .map_err(|e| anyhow::anyhow!("Failed to initialize websocket database: {}", e))?;
        let surrealdb_mem = surrealdb_init_mem().await
            .map_err(|e| anyhow::anyhow!("Failed to initialize memory database: {}", e))?;
        
        let state = AppState {
            surrealdb_ws,
            surrealdb_mem,
        };
        let repo = UsersRepository::new(&state);
        let user = repo.query_user_by_email(email.to_string()).await;
        match user {
            Ok(u) => Ok(Some(UserDto::from(&u))), // Corrected to use UserDto::from by reference
            Err(e) if e.to_string().contains("User not found") => Ok(None),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }

    #[allow(unused_variables)]
    async fn create_user_by_dto(&self, new_user: CreateUserDto) -> Result<UserDto> {
        let surrealdb_ws = surrealdb_init_ws().await
            .map_err(|e| anyhow::anyhow!("Failed to initialize websocket database: {}", e))?;
        let surrealdb_mem = surrealdb_init_mem().await
            .map_err(|e| anyhow::anyhow!("Failed to initialize memory database: {}", e))?;
        
        let state = AppState {
            surrealdb_ws,
            surrealdb_mem,
        };
        let repo = UsersRepository::new(&state);
        let email_clone = new_user.email.clone(); // Store email before moving new_user
        let user_schema = UsersSchema {
            email: new_user.email,
            password: new_user.password, // No unwrap_or_default needed
            fullname: new_user.fullname,
            phone_number: new_user.phone_number, // No unwrap_or_default needed
            is_active: new_user.is_active, // No unwrap_or needed
            role: make_thing(&ResourceEnum::Roles.to_string(), &new_user.role_id),
            ..Default::default()
        };
        match repo.query_create_user(user_schema).await {
            Ok(_msg) => { // msg is String, not UsersDetailQueryDto
                // Re-fetch the created user to get the full UsersDetailQueryDto
                let created_user = repo.query_user_by_email(email_clone).await?; // Use cloned email
                Ok(UserDto::from(&created_user)) // Corrected to use UserDto::from by reference
            },
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }

    async fn update_user_avatar(&self, email: &str, avatar_url: Option<String>) -> Result<()> {
        let surrealdb_ws = surrealdb_init_ws().await
            .map_err(|e| anyhow::anyhow!("Failed to initialize websocket database: {}", e))?;
        let surrealdb_mem = surrealdb_init_mem().await
            .map_err(|e| anyhow::anyhow!("Failed to initialize memory database: {}", e))?;
        
        let state = AppState {
            surrealdb_ws,
            surrealdb_mem,
        };
        let repo = UsersRepository::new(&state);
        
        // Get the existing user
        let mut user = repo.query_user_by_email(email.to_string()).await
            .map_err(|e| anyhow::anyhow!("Failed to get user: {}", e))?;
        
        // Update the avatar
        user.avatar = avatar_url;
        
        // Convert to schema and update
        let user_schema = UsersSchema::from(user);
        match repo.query_update_user(user_schema).await {
            Ok(_) => {
                info!("Successfully updated avatar for user: {}", email);
                Ok(())
            },
            Err(e) => Err(anyhow::anyhow!("Failed to update user avatar: {}", e)),
        }
    }
}
