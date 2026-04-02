use anyhow::Result;
use imphnen_libs::ENV;

#[derive(Debug, Clone)]
pub struct MinioConfig {
	pub endpoint: String,
	pub access_key: String,
	pub secret_key: String,
	pub bucket_name: String,
	pub region: String,
	pub secure: bool,
}

impl MinioConfig {
	pub fn from_env() -> Result<Self> {
		Ok(Self {
			endpoint: ENV.minio_endpoint.clone(),
			access_key: ENV.minio_access_key.clone(),
			secret_key: ENV.minio_secret_key.clone(),
			bucket_name: ENV.minio_bucket_name.clone(),
			region: ENV.minio_region.clone(),
			secure: ENV.minio_secure,
		})
	}

	pub fn endpoint_url(&self) -> String {
		if self.endpoint.starts_with("http://") || self.endpoint.starts_with("https://")
		{
			self.endpoint.clone()
		} else {
			let protocol = if self.secure { "https" } else { "http" };
			format!("{protocol}://{}", self.endpoint)
		}
	}
}
