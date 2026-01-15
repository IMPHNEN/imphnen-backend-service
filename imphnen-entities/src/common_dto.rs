use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MessageResponseDto {
	pub message: String,
	pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct MetaRequestDto {
	pub page: Option<u64>,
	pub per_page: Option<u64>,
	pub search: Option<String>,
	pub sort_by: Option<String>,
	pub order: Option<String>,
	pub filter: Option<String>,
	pub filter_by: Option<String>,
}

impl Default for MetaRequestDto {
	fn default() -> Self {
		MetaRequestDto {
			page: Some(1),
			per_page: Some(10),
			search: None,
			sort_by: None,
			order: None,
			filter: None,
			filter_by: None,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct MetaResponseDto {
	pub page: Option<u64>,
	pub per_page: Option<u64>,
	pub total: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ResponseSuccessDto<T: Serialize> {
	pub data: T,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ResponseListSuccessDto<T: Serialize> {
	pub data: T,
	pub meta: Option<MetaResponseDto>,
}


#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorDto {
    pub status: u16,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CountResult {
	pub count: u64,
}
