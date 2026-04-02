use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UploadRequest {
	pub filename: String,
	pub content_type: String,
	pub data: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UploadResponse {
	pub url: String,
}
