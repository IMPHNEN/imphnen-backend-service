use crate::require_permissions;
use std::sync::Arc;
use axum::{
    Extension, Json,
    extract::{Path, Multipart},
    http::HeaderMap,
    response::IntoResponse,
};
use paginator_axum::PaginationQuery;
use paginator_utils::PaginatorResponse;
use imphnen_libs::{AppState, MinioConfig, FileType, decode_base64_file, extract_content_type_from_data_url, create_minio_service_from_config};
use imphnen_utils::{ApiSuccess, ApiCreated, ApiPaginated, ApiMessage};
use imphnen_entities::{ResponseSuccessDto, ResponseListSuccessDto, PermissionsEnum, RolesDetailQueryDto};
use imphnen_utils::AppError;
use crate::users::domain::{UserEntity, UserService};
use super::dto::{
    FileUploadSchema, UsersActiveInactiveRequestDto, UsersCreateRequestDto, UsersDetailItemDto,
    UsersListItemDto, UsersUpdateRequestDto,
};
use imphnen_libs::hash_password;
use serde_json::json;
use tracing::error;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/v1/users",
    security(("Bearer" = [])),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page"),
        ("search" = Option<String>, Query, description = "Search keyword"),
        ("sort_by" = Option<String>, Query, description = "Sort by field"),
        ("order" = Option<String>, Query, description = "Order ASC or DESC"),
        ("filter" = Option<String>, Query, description = "Filter value"),
        ("filter_by" = Option<String>, Query, description = "Field to filter by"),
    ),
    responses(
        (status = 200, description = "[ADMIN] Get user list", body = ResponseListSuccessDto<Vec<UsersListItemDto>>)
    ),
    tag = "Users"
)]
pub async fn get_user_list(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn UserService>>,
    PaginationQuery(params): PaginationQuery,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers, state, [PermissionsEnum::ReadListUsers], {
        let result = service.list(params).await?;
        let mapped = PaginatorResponse {
            data: result.data.into_iter().map(UsersListItemDto::from).collect::<Vec<_>>(),
            meta: result.meta,
        };
        Ok(ApiPaginated(mapped))
    })
}

#[utoipa::path(
    get,
    path = "/v1/users/detail/{id}",
    security(("Bearer" = [])),
    params(("id" = String, Path, description = "User ID")),
    responses(
        (status = 200, description = "[ADMIN] Get user by ID", body = ResponseSuccessDto<UsersDetailItemDto>)
    ),
    tag = "Users"
)]
pub async fn get_user_by_id(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn UserService>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers, state, [PermissionsEnum::ReadDetailUsers], {
        Uuid::parse_str(&id)
            .map_err(|_| AppError::BadRequestError("Invalid User ID format".to_string()))?;
        let user = service.get(id).await?;
        if user.is_deleted {
            return Err(AppError::NotFoundError("User not found".to_string()));
        }
        Ok(ApiSuccess(UsersDetailItemDto::from(user)))
    })
}

#[utoipa::path(
    get,
    path = "/v1/users/me",
    security(("Bearer" = [])),
    responses(
        (status = 200, description = "[USER] Get current user", body = ResponseSuccessDto<UsersDetailItemDto>)
    ),
    tag = "Users"
)]
pub async fn get_user_me(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn UserService>>,
) -> Result<impl IntoResponse, AppError> {
    let (claims, _) = crate::permissions_guard(headers, axum::extract::Extension(state.clone()), vec![]).await?;
    let user = service.get_me(claims.user_id).await?;
    if user.is_deleted {
        return Err(AppError::NotFoundError("User not found".to_string()));
    }
    Ok(ApiSuccess(UsersDetailItemDto::from(user)))
}

#[utoipa::path(
    post,
    path = "/v1/users/create",
    security(("Bearer" = [])),
    request_body = UsersCreateRequestDto,
    responses(
        (status = 201, description = "[ADMIN] Create new user", body = ResponseSuccessDto<UsersDetailItemDto>)
    ),
    tag = "Users"
)]
pub async fn post_create_user(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn UserService>>,
    Json(payload): Json<UsersCreateRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers, state, [PermissionsEnum::CreateUsers], {
        let password_hash = hash_password(&payload.password)
            .map_err(|_| AppError::InternalServerError("Failed to hash password".to_string()))?;
        let role_id = payload.role_id.clone();
        let entity = UserEntity {
            id: Uuid::new_v4().to_string(),
            email: payload.email,
            fullname: payload.fullname,
            password: password_hash,
            is_active: payload.is_active,
            avatar: payload.avatar,
            role: RolesDetailQueryDto {
                id: role_id,
                ..Default::default()
            },
            ..Default::default()
        };
        let user = service.create(entity).await?;
        Ok(ApiCreated(UsersDetailItemDto::from(user)))
    })
}

#[utoipa::path(
    put,
    path = "/v1/users/update/{id}",
    security(("Bearer" = [])),
    params(("id" = String, Path, description = "User ID")),
    request_body = UsersUpdateRequestDto,
    responses(
        (status = 200, description = "[ADMIN] Update user")
    ),
    tag = "Users"
)]
pub async fn put_update_user(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn UserService>>,
    Path(id): Path<String>,
    Json(payload): Json<UsersUpdateRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers, state, [PermissionsEnum::UpdateUsers], {
        Uuid::parse_str(&id)
            .map_err(|_| AppError::BadRequestError("Invalid User ID format".to_string()))?;
        let current = service.get(id.clone()).await
            .map_err(|_| AppError::NotFoundError("User not found".to_string()))?;

        let password = if let Some(ref pw) = payload.password {
            hash_password(pw).unwrap_or_else(|_| current.password.clone())
        } else {
            current.password.clone()
        };

        let role_id = payload.role_id.clone().unwrap_or(current.role.id.clone());
        let entity = UserEntity {
            id: id.clone(),
            email: payload.email.unwrap_or(current.email),
            fullname: payload.fullname.unwrap_or(current.fullname),
            legal_name: payload.legal_name.or(current.legal_name),
            password,
            avatar: payload.avatar.or(current.avatar),
            is_active: payload.is_active.unwrap_or(current.is_active),
            is_deleted: current.is_deleted,
            role: RolesDetailQueryDto { id: role_id, ..current.role },
            profile_extension: payload.profile_extension.or(current.profile_extension),
            created_at: current.created_at,
            updated_at: current.updated_at,
            mentor_id: current.mentor_id,
        };
        let msg = service.update(entity).await?;
        Ok(ApiMessage::ok(&msg))
    })
}

#[utoipa::path(
    put,
    path = "/v1/users/update/me",
    security(("Bearer" = [])),
    request_body = UsersUpdateRequestDto,
    responses(
        (status = 200, description = "[USER] Update current user")
    ),
    tag = "Users"
)]
pub async fn put_update_user_me(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn UserService>>,
    Json(payload): Json<UsersUpdateRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    let (claims, _) = crate::permissions_guard(headers, axum::extract::Extension(state.clone()), vec![]).await?;
    let user_id = claims.user_id.clone();

    let current = service.get_me(user_id).await
        .map_err(|_| AppError::NotFoundError("User not found".to_string()))?;

    let password = if let Some(ref pw) = payload.password {
        hash_password(pw).unwrap_or_else(|_| current.password.clone())
    } else {
        current.password.clone()
    };

    let role_id = payload.role_id.clone().unwrap_or(current.role.id.clone());
    let entity = UserEntity {
        id: current.id.clone(),
        email: payload.email.unwrap_or(current.email),
        fullname: payload.fullname.unwrap_or(current.fullname),
        legal_name: payload.legal_name.or(current.legal_name),
        password,
        avatar: payload.avatar.or(current.avatar),
        is_active: payload.is_active.unwrap_or(current.is_active),
        is_deleted: current.is_deleted,
        role: RolesDetailQueryDto { id: role_id, ..current.role },
        profile_extension: payload.profile_extension.or(current.profile_extension),
        created_at: current.created_at,
        updated_at: current.updated_at,
        mentor_id: current.mentor_id,
    };
    let msg = service.update(entity).await?;
    Ok(ApiMessage::ok(&msg))
}

#[utoipa::path(
    put,
    path = "/v1/users/activate/{id}",
    security(("Bearer" = [])),
    params(("id" = String, Path, description = "User ID")),
    request_body = UsersActiveInactiveRequestDto,
    responses(
        (status = 200, description = "[ADMIN] Set user active status")
    ),
    tag = "Users"
)]
pub async fn patch_user_active_status(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn UserService>>,
    Path(id): Path<String>,
    Json(payload): Json<UsersActiveInactiveRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers, state, [PermissionsEnum::ActivateUsers], {
        Uuid::parse_str(&id)
            .map_err(|_| AppError::BadRequestError("Invalid User ID format".to_string()))?;
        let msg = service.set_active_status(id, payload.is_active).await?;
        Ok(ApiMessage::ok(&msg))
    })
}

#[utoipa::path(
    delete,
    path = "/v1/users/delete/{id}",
    security(("Bearer" = [])),
    params(("id" = String, Path, description = "User ID")),
    responses(
        (status = 200, description = "[ADMIN] Soft delete user")
    ),
    tag = "Users"
)]
pub async fn delete_user(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn UserService>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers, state, [PermissionsEnum::DeleteUsers], {
        Uuid::parse_str(&id)
            .map_err(|_| AppError::BadRequestError("Invalid User ID format".to_string()))?;
        let msg = service.delete(id).await?;
        Ok(ApiMessage::ok(&msg))
    })
}

#[utoipa::path(
    post,
    path = "/v1/users/upload",
    security(("Bearer" = [])),
    request_body(
        content = FileUploadSchema,
        description = "Upload file with multipart form data",
        content_type = "multipart/form-data"
    ),
    responses(
        (status = 200, description = "[USER] Upload file successfully", body = ResponseSuccessDto<serde_json::Value>),
        (status = 400, description = "[USER] Bad request"),
        (status = 401, description = "[USER] Unauthorized"),
        (status = 500, description = "[USER] Internal server error")
    ),
    tag = "Users"
)]
pub async fn upload_file(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let (claims, _) = crate::permissions_guard(headers, axum::extract::Extension(state.clone()), vec![]).await?;
    let user_id = claims.user_id.clone();

    let minio_config = MinioConfig::from_env()
        .map_err(|e| {
            error!("Failed to load MinIO config: {}", e);
            AppError::InternalServerError("MinIO configuration error".to_string())
        })?;
    let bucket_name = minio_config.bucket_name.clone();
    let minio_service = create_minio_service_from_config(minio_config).await
        .map_err(|e| {
            error!("Failed to initialize MinIO service: {}", e);
            AppError::InternalServerError("MinIO service initialization error".to_string())
        })?;

    let mut file_data: Option<Vec<u8>> = None;
    let mut filename: Option<String> = None;
    let mut content_type: Option<String> = None;

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "file" => {
                filename = field.file_name().map(|s| s.to_string());
                content_type = field.content_type().map(|s| s.to_string());
                match field.bytes().await {
                    Ok(bytes) => file_data = Some(bytes.to_vec()),
                    Err(e) => {
                        error!("Failed to read file data: {}", e);
                        return Err(AppError::BadRequestError("Failed to read file data".to_string()));
                    }
                }
            }
            "base64_data" => {
                let base64_str = field.text().await.unwrap_or_default();
                if !base64_str.is_empty() {
                    match decode_base64_file(&base64_str) {
                        Ok(decoded) => {
                            file_data = Some(decoded);
                            if let Some(ct) = extract_content_type_from_data_url(&base64_str) {
                                content_type = Some(ct);
                            }
                        }
                        Err(e) => {
                            error!("Failed to decode base64 data: {}", e);
                            return Err(AppError::BadRequestError("Invalid base64 data".to_string()));
                        }
                    }
                }
            }
            "filename" => filename = Some(field.text().await.unwrap_or_default()),
            "content_type" => content_type = Some(field.text().await.unwrap_or_default()),
            _ => {}
        }
    }

    let file_data = file_data
        .ok_or_else(|| AppError::BadRequestError("file data is required".to_string()))?;
    let filename = filename.unwrap_or_else(|| "unnamed_file".to_string());
    let content_type = content_type.unwrap_or_else(|| "application/octet-stream".to_string());

    let file_type = {
        let ft = FileType::from_content_type(&content_type);
        if matches!(ft, FileType::Unknown) { FileType::from_filename(&filename) } else { ft }
    };
    if matches!(file_type, FileType::Unknown) {
        return Err(AppError::BadRequestError("Unsupported file type".to_string()));
    }
    if !file_type.allowed_types().contains(&content_type.as_str()) {
        return Err(AppError::BadRequestError(format!("File type does not match content type '{content_type}'")));
    }
    if file_data.len() > file_type.max_size() {
        return Err(AppError::BadRequestError(format!(
            "File too large. Maximum size for {:?} is {} bytes",
            file_type,
            file_type.max_size()
        )));
    }

    let sanitized = user_id.replace('%', "").replace(':', "_").replace('@', "_at_").replace('.', "_");
    let folder = format!("{}/{sanitized}", file_type.as_folder());

    let object_path = minio_service
        .upload_file_with_deduplication(&file_data, &content_type, &folder, &filename)
        .await
        .map_err(|e| {
            error!("Failed to upload file: {}", e);
            AppError::InternalServerError(format!("Upload failed: {e}"))
        })?;

    let permanent_url = format!("https://cdn.asepharyana.tech/{}/{}", bucket_name, object_path);
    let response_data = json!({
        "filename": filename,
        "uploaded_path": object_path,
        "url": permanent_url,
        "size": file_data.len(),
        "content_type": content_type,
        "file_type": format!("{:?}", file_type).to_lowercase(),
        "user_id": user_id,
    });
    Ok(ApiSuccess(response_data))
}
