use anyhow::{Result, bail};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::helpers::decode_base64_file;
use crate::signing::{compute_header_auth, compute_presigned_url};
use crate::types::{get_file_extension, validate_file_type};

pub struct MinioService {
	pub(crate) endpoint: String,
	pub(crate) access_key: String,
	pub(crate) secret_key: String,
	pub(crate) bucket_name: String,
	pub(crate) region: String,
	pub(crate) client: reqwest::Client,
}

impl MinioService {
	pub async fn new(
		endpoint: &str,
		access_key: &str,
		secret_key: &str,
		bucket_name: &str,
		region: &str,
	) -> Result<Self> {
		Ok(Self {
			endpoint: endpoint.to_string(),
			access_key: access_key.to_string(),
			secret_key: secret_key.to_string(),
			bucket_name: bucket_name.to_string(),
			region: region.to_string(),
			client: reqwest::Client::new(),
		})
	}

	pub async fn upload_file_with_deduplication(
		&self,
		file_data: &[u8],
		content_type: &str,
		folder: &str,
		original_filename: &str,
	) -> Result<String> {
		validate_file_type(content_type, file_data)?;
		let mut hasher = Sha256::new();
		hasher.update(file_data);
		let file_hash = format!("{:x}", hasher.finalize());
		let short_hash = &file_hash[..16];
		if let Some(existing) =
			self.check_file_exists_by_hash(folder, short_hash).await?
		{
			return Ok(existing);
		}
		let ext = get_file_extension(original_filename);
		let unique_filename = format!("{folder}/{short_hash}-{}.{ext}", Uuid::new_v4());
		self
			.put_object(&unique_filename, file_data, content_type)
			.await?;
		tracing::info!("Uploaded {} bytes to {}", file_data.len(), unique_filename);
		Ok(unique_filename)
	}

	pub async fn upload_file(
		&self,
		file_data: &[u8],
		content_type: &str,
		folder: &str,
		original_filename: &str,
	) -> Result<String> {
		validate_file_type(content_type, file_data)?;
		let ext = get_file_extension(original_filename);
		let unique_filename = format!("{folder}/{}.{ext}", Uuid::new_v4());
		self
			.put_object(&unique_filename, file_data, content_type)
			.await?;
		tracing::info!("Uploaded {} bytes to {}", file_data.len(), unique_filename);
		Ok(unique_filename)
	}

	pub async fn upload_base64_file(
		&self,
		base64_data: &str,
		content_type: &str,
		folder: &str,
		original_filename: &str,
	) -> Result<String> {
		let file_data = decode_base64_file(base64_data)?;
		self
			.upload_file(&file_data, content_type, folder, original_filename)
			.await
	}

	pub async fn get_presigned_url(
		&self,
		object_name: &str,
		expiry_seconds: u32,
	) -> Result<String> {
		let host = self.strip_protocol();
		compute_presigned_url(
			host,
			&self.bucket_name,
			object_name,
			expiry_seconds,
			&self.access_key,
			&self.secret_key,
			&self.region,
		)
	}

	pub async fn check_file_exists_by_hash(
		&self,
		folder: &str,
		file_hash: &str,
	) -> Result<Option<String>> {
		let host = self.strip_protocol();
		let url = format!(
			"https://{host}/{}?list-type=2&prefix={folder}",
			self.bucket_name
		);
		let payload_hash = hex::encode(Sha256::digest(b""));
		let canonical_query =
			format!("list-type=2&prefix={}", urlencoding::encode(folder));
		let (auth_header, amz_date) = compute_header_auth(
			"GET",
			host,
			&format!("/{}", self.bucket_name),
			&canonical_query,
			&payload_hash,
			&self.access_key,
			&self.secret_key,
			&self.region,
		)?;

		let response = self
			.client
			.get(&url)
			.header("x-amz-date", &amz_date)
			.header("x-amz-content-sha256", &payload_hash)
			.header("Authorization", &auth_header)
			.send()
			.await?;

		if !response.status().is_success() {
			return Ok(None);
		}

		let body = response.text().await?;
		if body.contains(file_hash) {
			for line in body.lines() {
				if line.contains("<Key>")
					&& line.contains(file_hash)
					&& let Some(start) = line.find("<Key>")
					&& let Some(end) = line.find("</Key>")
				{
					return Ok(Some(line[start + 5..end].to_string()));
				}
			}
		}
		Ok(None)
	}

	pub async fn delete_file(&self, object_name: &str) -> Result<()> {
		let host = self.strip_protocol();
		let url = format!("https://{host}/{}/{object_name}", self.bucket_name);
		let payload_hash = hex::encode(Sha256::digest(b""));
		let canonical_uri = format!("/{}/{object_name}", self.bucket_name);
		let (auth_header, amz_date) = compute_header_auth(
			"DELETE",
			host,
			&canonical_uri,
			"",
			&payload_hash,
			&self.access_key,
			&self.secret_key,
			&self.region,
		)?;

		let response = self
			.client
			.delete(&url)
			.header("Host", host)
			.header("x-amz-date", &amz_date)
			.header("x-amz-content-sha256", &payload_hash)
			.header("Authorization", &auth_header)
			.send()
			.await?;

		if !response.status().is_success() {
			let status = response.status();
			let error_body = response.text().await?;
			bail!(
				"Failed to delete from MinIO. Status: {}. Message: {}",
				status,
				error_body
			);
		}

		tracing::info!("Deleted file: {}", object_name);
		Ok(())
	}

	async fn put_object(
		&self,
		object_name: &str,
		file_data: &[u8],
		content_type: &str,
	) -> Result<()> {
		let host = self.strip_protocol();
		let url = format!("https://{host}/{}/{object_name}", self.bucket_name);
		let payload_hash = "UNSIGNED-PAYLOAD".to_string();
		let canonical_uri = format!("/{}/{object_name}", self.bucket_name);
		let (auth_header, amz_date) = compute_header_auth(
			"PUT",
			host,
			&canonical_uri,
			"",
			&payload_hash,
			&self.access_key,
			&self.secret_key,
			&self.region,
		)?;

		let response = self
			.client
			.put(&url)
			.header("x-amz-date", &amz_date)
			.header("x-amz-content-sha256", &payload_hash)
			.header("Authorization", &auth_header)
			.header("Content-Type", content_type)
			.header("X-Forwarded-Proto", "https")
			.header("X-Forwarded-Host", host)
			.body(file_data.to_vec())
			.send()
			.await?;

		if !response.status().is_success() {
			let status = response.status();
			let error_body = response.text().await?;
			bail!(
				"Failed to upload to MinIO. Status: {}. Message: {}",
				status,
				error_body
			);
		}
		Ok(())
	}

	fn strip_protocol(&self) -> &str {
		self
			.endpoint
			.trim_start_matches("https://")
			.trim_start_matches("http://")
	}
}
