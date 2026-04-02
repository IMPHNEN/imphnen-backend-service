use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose};

use crate::config::MinioConfig;
use crate::service::MinioService;

pub async fn create_minio_service_from_config(
	config: MinioConfig,
) -> Result<MinioService> {
	MinioService::new(
		&config.endpoint,
		&config.access_key,
		&config.secret_key,
		&config.bucket_name,
		&config.region,
	)
	.await
}

pub fn decode_base64_file(base64_data: &str) -> Result<Vec<u8>> {
	let clean_data = if base64_data.contains(',') {
		base64_data.split(',').nth(1).unwrap_or(base64_data)
	} else {
		base64_data
	};
	general_purpose::STANDARD
		.decode(clean_data)
		.map_err(|e| anyhow!("Failed to decode base64 data: {}", e))
}

pub fn extract_content_type_from_data_url(data_url: &str) -> Option<String> {
	if data_url.starts_with("data:")
		&& let Some(type_part) = data_url.split(';').next()
	{
		return Some(type_part.replace("data:", ""));
	}
	None
}
