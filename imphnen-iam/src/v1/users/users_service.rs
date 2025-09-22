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
use imphnen_utils::make_thing_from_enum;
use uuid::Uuid;
use std::pin::Pin;
use std::future::Future;
use anyhow::Result;

use tracing::info;
use tracing::error;
use crate::v1::users::users_dto::{UsersDetailItemDto as UserDto, UsersCreateRequestDto as CreateUserDto};
use serde_json::json;


pub trait UsersServiceTrait: Send + Sync + 'static {
    fn get_user_list(state: &AppState, meta: MetaRequestDto) -> Pin<Box<dyn Future<Output = Response> + Send>>;
    fn get_user_by_id(state: &AppState, id: String) -> Pin<Box<dyn Future<Output = Response> + Send>>;
    fn get_user_me(claims: imphnen_libs::jsonwebtoken::Claims, state: &AppState) -> Pin<Box<dyn Future<Output = Response> + Send>>;
    fn create_user(state: &AppState, new_user: UsersCreateRequestDto) -> Pin<Box<dyn Future<Output = Response> + Send>>;
    fn update_user(state: &AppState, id: String, user: UsersUpdateRequestDto) -> Pin<Box<dyn Future<Output = Response> + Send>>;
    fn update_user_me(claims: imphnen_libs::jsonwebtoken::Claims, state: &AppState, user: UsersUpdateRequestDto) -> Pin<Box<dyn Future<Output = Response> + Send>>;
    fn set_user_active_status(state: &AppState, id: String, payload: UsersActiveInactiveRequestDto) -> Pin<Box<dyn Future<Output = Response> + Send>>;
    fn update_user_password(state: &AppState, email: String, payload: UsersSetNewPasswordRequestDto) -> Pin<Box<dyn Future<Output = Response> + Send>>;
    fn get_user_by_mentor_id(state: &AppState, mentor_id: String) -> Pin<Box<dyn Future<Output = Response> + Send>>;
    fn delete_user(state: &AppState, id: String) -> Pin<Box<dyn Future<Output = Response> + Send>>;
    fn get_user_by_id_internal(&self, id: &surrealdb::sql::Thing, state: &AppState) -> Pin<Box<dyn Future<Output = Result<UsersDetailQueryDto>> + Send>>;
 
     fn get_user_by_email(&self, email: &str, state: &AppState) -> Pin<Box<dyn Future<Output = Result<Option<UserDto>>> + Send>>;
     fn create_user_by_dto(&self, new_user: CreateUserDto, state: &AppState) -> Pin<Box<dyn Future<Output = Result<UserDto>> + Send>>;
     fn update_user_avatar(email: &str, avatar_url: Option<String>, state: &AppState) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>;
     fn upload_file(state: &AppState, user_id: String, multipart: Multipart) -> Pin<Box<dyn Future<Output = Response> + Send>>;
 }
 
 #[derive(Clone)]
  pub struct UsersService;
  
  impl UsersService {
  }
  
  
  impl UsersServiceTrait for UsersService {
     fn get_user_by_id_internal(&self, id: &surrealdb::sql::Thing, state: &AppState) -> Pin<Box<dyn Future<Output = Result<UsersDetailQueryDto>> + Send>> {
        let id = id.to_owned();
        let state = state.to_owned();
        Box::pin(async move {
         let repo = crate::UsersRepository::new(&state);
         repo.query_user_by_id(&id).await
        })
     }
  	fn get_user_list(state: &AppState, meta: MetaRequestDto) -> Pin<Box<dyn Future<Output = Response> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
		let repo = UsersRepository::new(&state);
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
        })
	}
 
	fn get_user_by_id(state: &AppState, id: String) -> Pin<Box<dyn Future<Output = Response> + Send>> {
        let state = state.to_owned();
        let id = id.to_owned();
        Box::pin(async move {
		if Uuid::parse_str(&id).is_err() {
            return common_response(StatusCode::BAD_REQUEST, "Invalid User ID format");
        }
		let repo = UsersRepository::new(&state);
		let thing_id = make_thing_from_enum(ResourceEnum::Users, &id);
		match repo.query_user_by_id(&thing_id).await {
			Ok(user) if !user.is_deleted => success_response(ResponseSuccessDto {
				data: UserDto::from(&user), // Corrected to use UserDto::from by reference
			}),
			Ok(_) => common_response(StatusCode::NOT_FOUND, "User not found"),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
        })
	}
 
	fn get_user_me(claims: imphnen_libs::jsonwebtoken::Claims, state: &AppState) -> Pin<Box<dyn Future<Output = Response> + Send>> {
        let claims = claims.to_owned();
        let state = state.to_owned();
        Box::pin(async move {
		let repo = UsersRepository::new(&state);
		let thing_id = make_thing_from_enum(ResourceEnum::Users, &claims.user_id);
		match repo.query_user_by_id(&thing_id).await {
			Ok(user) if !user.is_deleted => success_response(ResponseSuccessDto {
				data: UserDto::from(&user),
			}),
			Ok(_) => common_response(StatusCode::NOT_FOUND, "User not found"),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
        })
	}
 
	fn create_user(
		state: &AppState,
		new_user: UsersCreateRequestDto,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
		if let Err((status, message)) = validate_request(&new_user) {
			return common_response(status, &message);
		}
		let repo = UsersRepository::new(&state);
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
        })
	}
 
	fn update_user(
		state: &AppState,
		id: String,
		user: UsersUpdateRequestDto,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
        let state = state.to_owned();
        let id = id.to_owned();
        Box::pin(async move {
		if Uuid::parse_str(&id).is_err() {
            return common_response(StatusCode::BAD_REQUEST, "Invalid User ID format");
        }
		let repo = UsersRepository::new(&state);
		if let Err((status, message)) = validate_request(&user) {
			return common_response(status, &message);
		}
		
		// Get current user data first
		let thing_id = make_thing_from_enum(ResourceEnum::Users, &id);
		let current_user = match repo.query_user_by_id(&thing_id).await {
			Ok(user) => user,
			Err(_) => return common_response(StatusCode::NOT_FOUND, "User not found"),
		};
		
		let updated_user = UsersSchema::partial_update(current_user, user);
		match repo.query_update_user(updated_user).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
        })
	}
 
	fn update_user_me(
		claims: imphnen_libs::jsonwebtoken::Claims,
		state: &AppState,
		user_update_dto: UsersUpdateRequestDto,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
        let claims = claims.to_owned();
        let state = state.to_owned();
        Box::pin(async move {
		let repo = UsersRepository::new(&state);
		
		let thing_id = make_thing_from_enum(ResourceEnum::Users, &claims.user_id);
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
        })
	}
 
	fn set_user_active_status(
		state: &AppState,
		id: String,
		payload: UsersActiveInactiveRequestDto,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
        let state = state.to_owned();
        let id = id.to_owned();
        Box::pin(async move {
		if Uuid::parse_str(&id).is_err() {
            return common_response(StatusCode::BAD_REQUEST, "Invalid User ID format");
        }
		let repo = UsersRepository::new(&state);
		let thing_id = make_thing_from_enum(ResourceEnum::Users, &id);
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
        })
	}
 
	fn update_user_password(
		state: &AppState,
		email: String,
		payload: UsersSetNewPasswordRequestDto,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
        let state = state.to_owned();
        let email = email.to_owned();
        Box::pin(async move {
		let repo = UsersRepository::new(&state);
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
        })
	}
 
	fn get_user_by_mentor_id(
		state: &AppState,
		mentor_id: String,
	) -> Pin<Box<dyn Future<Output = Response> + Send>> {
        let state = state.to_owned();
        let mentor_id = mentor_id.to_owned();
        Box::pin(async move {
		let repo = UsersRepository::new(&state);
		let thing_id = make_thing_from_enum(ResourceEnum::Mentors, &mentor_id);
		match repo.query_user_by_id(&thing_id).await {
			Ok(user) if !user.is_deleted => success_response(ResponseSuccessDto {
				data: UserDto::from(&user), // Corrected to use UserDto::from by reference
			}),
			Ok(_) => common_response(StatusCode::NOT_FOUND, "User not found"),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
        })
	}
 
	fn delete_user(state: &AppState, id: String) -> Pin<Box<dyn Future<Output = Response> + Send>> {
        let state = state.to_owned();
        let id = id.to_owned();
        Box::pin(async move {
		if Uuid::parse_str(&id).is_err() {
            return common_response(StatusCode::BAD_REQUEST, "Invalid User ID format");
        }
		let repo = UsersRepository::new(&state);
		let thing_id = make_thing_from_enum(ResourceEnum::Users, &id);
		if repo.query_user_by_id(&thing_id).await.is_err() {
			return common_response(StatusCode::BAD_REQUEST, "User not found");
		}
		match repo.query_delete_user(id).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
        })
	}
 
    fn get_user_by_email(&self, email: &str, state: &AppState) -> Pin<Box<dyn Future<Output = Result<Option<UserDto>>> + Send>> {
        let email = email.to_owned();
        let state = state.to_owned();
        Box::pin(async move {
        let repo = UsersRepository::new(&state);
        let user = repo.query_user_by_email(email.to_string()).await;
        match user {
            Ok(u) => Ok(Some(UserDto::from(&u))),
            Err(e) if e.to_string().contains("User not found") => Ok(None),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
        })
    }
 
    fn create_user_by_dto(&self, new_user: CreateUserDto, state: &AppState) -> Pin<Box<dyn Future<Output = Result<UserDto>> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
        let repo = UsersRepository::new(&state);
        let email_clone = new_user.email.clone();
        let user_schema = UsersSchema {
            email: new_user.email,
            password: new_user.password,
            fullname: new_user.fullname,
            phone_number: new_user.phone_number,
            is_active: new_user.is_active,
            avatar: new_user.avatar,
            role: make_thing_from_enum(ResourceEnum::Roles, &new_user.role_id),
            ..Default::default()
        };
        match repo.query_create_user(user_schema).await {
            Ok(_msg) => {
                let created_user = repo.query_user_by_email(email_clone).await?;
                Ok(UserDto::from(&created_user))
            },
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
        })
    }
 
    fn update_user_avatar(email: &str, avatar_url: Option<String>, state: &AppState) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        let email = email.to_owned();
        let avatar_url = avatar_url.to_owned();
        let state = state.to_owned();
        Box::pin(async move {
        let repo = UsersRepository::new(&state);
        
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
        })
    }
 
    fn upload_file(state: &AppState, user_id: String, mut multipart: Multipart) -> Pin<Box<dyn Future<Output = Response> + Send>> {
        let state = state.to_owned();
        let user_id = user_id.to_owned();
        Box::pin(async move {
            info!("Entering upload_file function for user_id: {}", user_id);

            // Initialize MinIO configuration
            let minio_config = match MinioConfig::from_env() {
                Ok(config) => {
                    info!("MinIO config loaded successfully.");
                    config
                },
                Err(e) => {
                    error!("Failed to load MinIO config: {}", e);
                    return common_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "MinIO configuration error",
                    );
                }
            };

            // Store bucket name before minio_config is moved
            let bucket_name = minio_config.bucket_name.clone();
            info!("MinIO bucket name: {}", bucket_name);

            // Initialize MinIO service
            let minio_service = match create_minio_service_from_config(minio_config).await {
                Ok(service) => {
                    info!("MinIO service initialized successfully.");
                    service
                },
                Err(e) => {
                    error!("Failed to initialize MinIO service: {}", e);
                    return common_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "MinIO service initialization error",
                    );
                }
            };

            // Get actual user data from database using user_id (which is a UUID)
            let repo = UsersRepository::new(&state);
            let thing_id = make_thing_from_enum(ResourceEnum::Users, &user_id);
            let user_data = match repo.query_user_by_id(&thing_id).await {
                Ok(user) => {
                    info!("Found user in DB. User ID: {}, Email: {}", user.id.id.to_raw(), user.email);
                    user
                }
                Err(e) => {
                    error!("Failed to find user in DB for ID {}: {}", user_id, e);
                    return common_response(
                        StatusCode::NOT_FOUND,
                        "User not found",
                    );
                }
            };
            let actual_user_id = user_data.id.id.to_raw();
            let user_email = user_data.email;

            let mut file_data: Option<Vec<u8>> = None;
            let mut filename: Option<String> = None;
            let mut content_type: Option<String> = None;

            info!("Starting multipart form processing.");
            while let Some(field) = multipart.next_field().await.unwrap_or(None) {
                let name = field.name().unwrap_or("").to_string();
                info!("Processing multipart field: {}", name);
                
                match name.as_str() {
                    "file" => {
                        filename = field.file_name().map(|s| s.to_string());
                        content_type = field.content_type().map(|s| s.to_string());
                        info!("Detected file field. Filename: {:?}, Content-Type: {:?}", filename, content_type);
                        
                        match field.bytes().await {
                            Ok(bytes) => {
                                file_data = Some(bytes.to_vec());
                                info!("Successfully read file data, size: {} bytes", file_data.as_ref().map_or(0, |d| d.len()));
                            },
                            Err(e) => {
                                error!("Failed to read file data from multipart: {}", e);
                                return common_response(
                                    StatusCode::BAD_REQUEST,
                                    "Failed to read file data",
                                );
                            }
                        }
                    }
                    "base64_data" => {
                        let base64_str = field.text().await.unwrap_or_default();
                        info!("Detected base64_data field, length: {}", base64_str.len());
                        if !base64_str.is_empty() {
                            match decode_base64_file(&base64_str) {
                                Ok(decoded_data) => {
                                    file_data = Some(decoded_data);
                                    info!("Successfully decoded base64 data, size: {} bytes", file_data.as_ref().map_or(0, |d| d.len()));
                                    if let Some(detected_type) = extract_content_type_from_data_url(&base64_str) {
                                        content_type = Some(detected_type);
                                        info!("Detected content type from base64 data URL: {}", content_type.as_ref().unwrap());
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to decode base64 data: {}", e);
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
                        info!("Received filename from field: {:?}", filename);
                    }
                    "content_type" => {
                        content_type = Some(field.text().await.unwrap_or_default());
                        info!("Received content_type from field: {:?}", content_type);
                    }
                    _ => {
                        info!("Skipping unknown multipart field: {}", name);
                    }
                }
            }
            info!("Finished multipart form processing.");

            // Validate required fields
            let file_data = match file_data {
                Some(data) => data,
                None => {
                    error!("File data is missing after multipart processing.");
                    return common_response(
                        StatusCode::BAD_REQUEST,
                        "file data is required",
                    );
                }
            };
            info!("File data extracted, size: {} bytes.", file_data.len());

            let filename = filename.unwrap_or_else(|| {
                info!("Filename not provided, defaulting to 'unnamed_file'.");
                "unnamed_file".to_string()
            });
            let content_type = content_type.unwrap_or_else(|| {
                info!("Content type not provided, defaulting to 'application/octet-stream'.");
                "application/octet-stream".to_string()
            });
            info!("Final filename: {}, Content-Type: {}", filename, content_type);

            // Auto-detect file type based on content type and filename
            let file_type = FileType::from_content_type(&content_type);
            let file_type = if matches!(file_type, FileType::Unknown) {
                info!("Content type detection failed, trying from filename.");
                FileType::from_filename(&filename)
            } else {
                file_type
            };
            info!("Detected file type: {:?}", file_type);

            // Validate file type is supported
            if matches!(file_type, FileType::Unknown) {
                error!("Unsupported file type detected: {:?}", file_type);
                return common_response(
                    StatusCode::BAD_REQUEST,
                    "Unsupported file type. Supported types: JPEG, PNG, WEBP, GIF, PDF, DOC, DOCX",
                );
            }

            // Validate file type matches content type
            if !file_type.allowed_types().contains(&content_type.as_str()) {
                error!("File type '{:?}' does not match content type '{}'.", file_type, content_type);
                return common_response(
                    StatusCode::BAD_REQUEST,
                    &format!("File type '{:?}' does not match content type '{}'", file_type, content_type),
                );
            }

            // Validate file size
            if file_data.len() > file_type.max_size() {
                error!("File too large. Current size: {} bytes, Max size for {:?}: {} bytes", 
                    file_data.len(), file_type, file_type.max_size());
                return common_response(
                    StatusCode::BAD_REQUEST,
                    &format!("File too large. Maximum size for {:?} is {} bytes", 
                        file_type, file_type.max_size()),
                );
            }
            info!("File size validated: {} bytes.", file_data.len());

            // Create secure upload path with user ID (sanitized for filesystem)
            let sanitized_user_id = user_email
                .replace("%", "")
                .replace(":", "_")
                .replace("@", "_at_")
                .replace(".", "_");
            info!("Sanitized user ID for folder path: {}", sanitized_user_id);
            
            let folder = format!("{}/{}", file_type.as_folder(), sanitized_user_id);
            info!("Upload folder: {}", folder);

            // Upload file to MinIO with deduplication
            info!("Attempting to upload file to MinIO.");
            match minio_service.upload_file_with_deduplication(&file_data, &content_type, &folder, &filename).await {
                Ok(object_path) => {
                    info!("File uploaded successfully to MinIO. Object path: {}", object_path);
                    // Create permanent URL (no expiration)
                    let permanent_url = format!("https://cdn.asepharyana.tech/{}/{}", 
                        bucket_name, object_path);
                    info!("Permanent URL: {}", permanent_url);

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
                    error!("Failed to upload file to MinIO: {}", e);
                    common_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        &format!("Upload failed: {}", e),
                    )
                }
            }
        })
    }
}