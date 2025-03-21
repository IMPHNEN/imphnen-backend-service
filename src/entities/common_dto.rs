use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};
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

pub type SurrealClient = Surreal<Client>;
pub type RedisClient = redis::Client;

#[derive(Clone)]
pub struct AppState {
	pub surrealdb: SurrealClient,
	pub redisdb: RedisClient,
}
