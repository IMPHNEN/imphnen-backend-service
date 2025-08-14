use super::{
	UsersActiveInactiveRequestDto, UsersCreateRequestDto,
	UsersSetNewPasswordRequestDto, UsersUpdateRequestDto,
    users_dto::UsersDetailQueryDto, // Add this line
};
use crate::{
	AppState, MetaRequestDto, ResponseListSuccessDto, UsersRepository, UsersSchema,
};
use crate::{
	ResponseSuccessDto, common_response, success_list_response,
	success_response, validate_request,
};
use axum::{http::StatusCode, response::Response, extract::Multipart};
use imphnen_libs::{ResourceEnum, hash_password, verify_password, MinioConfig, FileType, decode_base64_file, extract_content_type_from_data_url, create_minio_service_from_config};
use imphnen_utils::make_thing;
use uuid::Uuid;
use anyhow::Result;
use async_trait::async_trait;
use tracing::info;
use crate::v1::users::users_dto::{UsersDetailItemDto as UserDto, UsersCreateRequestDto as CreateUserDto};
use serde_json::json;

#[async_trait]
pub trait UsersServiceTrait: Send + Sync + 'static {
    async fn get_user_list(state: &AppState, meta: MetaRequestDto) -> Response;
    async fn get_user_by_id(state: &AppState, id: String) -> Response;
    async fn get_user_me(claims: imphnen_libs::jsonwebtoken::Claims, state: &AppState) -> Response;
    async fn create_user(state: &AppState, new_user: UsersCreateRequestDto) -> Response;
    async fn update_user(state: &AppState, id: String, user: UsersUpdateRequestDto) -> Response;
    async fn update_user_me(claims: imphnen_libs::jsonwebtoken::Claims, state: &AppState, user: UsersUpdateRequestDto) -> Response;
    async fn set_user_active_status(state: &AppState, id: String, payload: UsersActiveInactiveRequestDto) -> Response;
    async fn update_user_password(state: &AppState, email: String, payload: UsersSetNewPasswordRequestDto) -> Response;
    async fn get_user_by_mentor_id(state: &AppState, mentor_id: String) -> Response;
    async fn delete_user(state: &AppState, id: String) -> Response;
    async fn get_user_by_id_internal(&self, id: &surrealdb::sql::Thing, state: &AppState) -> Result<UsersDetailQueryDto>;
 
     async fn get_user_by_email(&self, email: &str, state: &AppState) -> Result<Option<UserDto>>;
     async fn create_user_by_dto(&self, new_user: CreateUserDto, state: &AppState) -> Result<UserDto>;
     async fn update_user_avatar(&self, email: &str, avatar_url: Option<String>, state: &AppState) -> Result<()>;
     async fn upload_file(state: &AppState, user_id: String, multipart: Multipart) -> Response;
 }
 
 #[derive(Clone)]
  pub struct UsersService;
  
  impl UsersService {
  }
  
  #[async_trait]
  impl UsersServiceTrait for UsersService {
     async fn get_user_by_id_internal(&self, id: &surrealdb::sql::Thing, state: &AppState) -> Result<UsersDetailQueryDto> {
         let repo = crate::UsersRepository::new(state);
         repo.query_user_by_id(id).await
     }
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

	async fn get_user_me(claims: imphnen_libs::jsonwebtoken::Claims, state: &AppState) -> Response {
		let repo = UsersRepository::new(state);
		let thing_id = make_thing(&ResourceEnum::Users.to_string(), &claims.user_id);
		match repo.query_user_by_id(&thing_id).await {
			Ok(user) if !user.is_deleted => success_response(ResponseSuccessDto {
				data: UserDto::from(&user),
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
		
		// Get current user data first
		let thing_id = make_thing(&ResourceEnum::Users.to_string(), &id);
		let current_user = match repo.query_user_by_id(&thing_id).await {
			Ok(user) => user,
			Err(_) => return common_response(StatusCode::NOT_FOUND, "User not found"),
		};
		
		let updated_user = UsersSchema::partial_update(current_user, user);
		match repo.query_update_user(updated_user).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	async fn update_user_me(
		claims: imphnen_libs::jsonwebtoken::Claims,
		state: &AppState,
		user_update_dto: UsersUpdateRequestDto,
	) -> Response {
		let repo = UsersRepository::new(state);
		
		let thing_id = make_thing(&ResourceEnum::Users.to_string(), &claims.user_id);
		let user_data = match repo.query_user_by_id(&thing_id).await {
			Ok(user) => user,
			Err(_) => return common_response(StatusCode::NOT_FOUND, "User not found"),
		};
		
		if let Err((status, message)) = validate_request(&user_update_dto) {
			return common_response(status, &message);
		}
		
		let updated_user = UsersSchema::partial_update(user_data, user_update_dto);
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

    async fn get_user_by_email(&self, email: &str, state: &AppState) -> Result<Option<UserDto>> {
        let repo = UsersRepository::new(state);
        let user = repo.query_user_by_email(email.to_string()).await;
        match user {
            Ok(u) => Ok(Some(UserDto::from(&u))),
            Err(e) if e.to_string().contains("User not found") => Ok(None),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }

    async fn create_user_by_dto(&self, new_user: CreateUserDto, state: &AppState) -> Result<UserDto> {
        let repo = UsersRepository::new(state);
        let email_clone = new_user.email.clone();
        let user_schema = UsersSchema {
            email: new_user.email,
            password: new_user.password,
            fullname: new_user.fullname,
            phone_number: new_user.phone_number,
            is_active: new_user.is_active,
            avatar: new_user.avatar,
            role: make_thing(&ResourceEnum::Roles.to_string(), &new_user.role_id),
            ..Default::default()
        };
        match repo.query_create_user(user_schema).await {
            Ok(_msg) => {
                let created_user = repo.query_user_by_email(email_clone).await?;
                Ok(UserDto::from(&created_user))
            },
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }

    async fn update_user_avatar(&self, email: &str, avatar_url: Option<String>, state: &AppState) -> Result<()> {
        let repo = UsersRepository::new(state);
        
        let mut user = repo.query_user_by_email(email.to_string()).await
            .map_err(|e| anyhow::anyhow!("Failed to get user: {}", e))?;
        
        user.avatar = avatar_url;
        
        let user_schema = UsersSchema::from(user);
        match repo.query_update_user(user_schema).await {
            Ok(_) => {
                info!("Successfully updated avatar for user: {}", email);
                Ok(())
            },
            Err(e) => Err(anyhow::anyhow!("Failed to update user avatar: {}", e)),
        }
    }

    async fn upload_file(state: &AppState, user_id: String, mut multipart: Multipart) -> Response {
        // Initialize MinIO configuration
        let minio_config = match MinioConfig::from_env() {
            Ok(config) => config,
            Err(e) => {
                log::error!("Failed to load MinIO config: {}", e);
                return common_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "MinIO configuration error",
                );
            }
        };

        // Store bucket name before minio_config is moved
        let bucket_name = minio_config.bucket_name.clone();

        // Initialize MinIO service
        let minio_service = match create_minio_service_from_config(minio_config).await {
            Ok(service) => service,
            Err(e) => {
                log::error!("Failed to initialize MinIO service: {}", e);
                return common_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "MinIO service initialization error",
                );
            }
        };

        // Extract email from user_id (which contains email in SurrealDB format)
        let user_email = user_id
            .replace("app_users:", "")
            .replace("⟨", "")
            .replace("⟩", "");
            
        // Get actual user data from database to get real user ID
        let repo = UsersRepository::new(state);
        let (actual_user_id, user_email) = match repo.query_user_by_email(user_email.clone()).await {
            Ok(user) => {
                // Extract the actual ID from the user record
                let actual_id = user.id.id.to_raw();
                (actual_id, user.email)
            }
            Err(_) => {
                return common_response(
                    StatusCode::NOT_FOUND,
                    "User not found",
                );
            }
        };

        let mut file_data: Option<Vec<u8>> = None;
        let mut filename: Option<String> = None;
        let mut content_type: Option<String> = None;

        // Process multipart form
        while let Some(field) = multipart.next_field().await.unwrap_or(None) {
            let name = field.name().unwrap_or("").to_string();
            
            match name.as_str() {
                "file" => {
                    filename = field.file_name().map(|s| s.to_string());
                    content_type = field.content_type().map(|s| s.to_string());
                    
                    match field.bytes().await {
                        Ok(bytes) => file_data = Some(bytes.to_vec()),
                        Err(e) => {
                            log::error!("Failed to read file data: {}", e);
                            return common_response(
                                StatusCode::BAD_REQUEST,
                                "Failed to read file data",
                            );
                        }
                    }
                }
                "base64_data" => {
                    // Handle base64 data from frontend
                    let base64_str = field.text().await.unwrap_or_default();
                    if !base64_str.is_empty() {
                        match decode_base64_file(&base64_str) {
                            Ok(decoded_data) => {
                                file_data = Some(decoded_data);
                                // Extract content type from data URL if present
                                if let Some(detected_type) = extract_content_type_from_data_url(&base64_str) {
                                    content_type = Some(detected_type);
                                }
                            }
                            Err(e) => {
                                log::error!("Failed to decode base64 data: {}", e);
                                return common_response(
                                    StatusCode::BAD_REQUEST,
                                    "Invalid base64 data",
                                );
                            }
                        }
                    }
                }
                "filename" => {
                    filename = Some(field.text().await.unwrap_or_default());
                }
                "content_type" => {
                    content_type = Some(field.text().await.unwrap_or_default());
                }
                _ => {
                    // Skip unknown fields
                }
            }
        }

        // Validate required fields
        let file_data = match file_data {
            Some(data) => data,
            None => {
                return common_response(
                    StatusCode::BAD_REQUEST,
                    "file data is required",
                );
            }
        };

        let filename = filename.unwrap_or_else(|| "unnamed_file".to_string());
        let content_type = content_type.unwrap_or_else(|| "application/octet-stream".to_string());

        // Auto-detect file type based on content type and filename
        let file_type = FileType::from_content_type(&content_type);
        let file_type = if matches!(file_type, FileType::Unknown) {
            FileType::from_filename(&filename)
        } else {
            file_type
        };

        // Validate file type is supported
        if matches!(file_type, FileType::Unknown) {
            return common_response(
                StatusCode::BAD_REQUEST,
                "Unsupported file type. Supported types: JPEG, PNG, WEBP, GIF, PDF, DOC, DOCX",
            );
        }

        // Validate file type matches content type
        if !file_type.allowed_types().contains(&content_type.as_str()) {
            return common_response(
                StatusCode::BAD_REQUEST,
                &format!("File type '{}' does not match content type '{:?}'", content_type, file_type),
            );
        }

        // Validate file size
        if file_data.len() > file_type.max_size() {
            return common_response(
                StatusCode::BAD_REQUEST,
                &format!("File too large. Maximum size for {:?} is {} bytes", 
                    file_type, file_type.max_size()),
            );
        }

        // Create secure upload path with user ID (sanitized for filesystem)
        let sanitized_user_id = user_email
            .replace("%", "")
            .replace(":", "_")
            .replace("@", "_at_")
            .replace(".", "_");
        
        let folder = format!("{}/{}", file_type.as_folder(), sanitized_user_id);

        // Upload file to MinIO with deduplication
        match minio_service.upload_file_with_deduplication(&file_data, &content_type, &folder, &filename).await {
            Ok(object_path) => {
                // Create permanent URL (no expiration)
                let permanent_url = format!("https://cdn.asepharyana.tech/{}/{}", 
                    bucket_name, object_path);

                let response_data = json!({
                    "filename": filename,
                    "original_filename": filename,
                    "uploaded_path": object_path,
                    "url": permanent_url,
                    "size": file_data.len(),
                    "content_type": content_type,
                    "file_type": format!("{:?}", file_type).to_lowercase(),
                    "user_id": actual_user_id,
                    "email": user_email
                });

                success_response(ResponseSuccessDto {
                    data: response_data,
                })
            }
            Err(e) => {
                log::error!("Failed to upload file: {}", e);
                common_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &format!("Upload failed: {}", e),
                )
            }
        }
    }
}
