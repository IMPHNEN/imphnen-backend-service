use std::io::Cursor;
use anyhow::{Result, bail};
use base64::{Engine as _, engine::general_purpose};
use minio::s3::args::{BucketExistsArgs, GetPresignedObjectUrlArgs, MakeBucketArgs, PutObjectArgs};
use minio::s3::client::Client;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use uuid::Uuid;

pub struct MinioService {
    client: Client,
    bucket_name: String,
}

impl MinioService {
    pub async fn new(endpoint: &str, access_key: &str, secret_key: &str, bucket_name: &str) -> Result<Self> {
        let base_url = endpoint.parse::<BaseUrl>()?;
        let static_provider = StaticProvider::new(access_key, secret_key, None);
        let client = Client::new(
            base_url.clone(),
            Some(Box::new(static_provider)),
            None,
            None,
        )?;

        let service = Self {
            client,
            bucket_name: bucket_name.to_string(),
        };

        // Ensure bucket exists
        service.ensure_bucket_exists().await?;

        Ok(service)
    }

    async fn ensure_bucket_exists(&self) -> Result<()> {
        let exists_args = BucketExistsArgs::new(&self.bucket_name)?;
        let exists = self.client.bucket_exists(&exists_args).await?;
        
        if !exists {
            let make_bucket_args = MakeBucketArgs::new(&self.bucket_name)?;
            self.client.make_bucket(&make_bucket_args).await?;
        }
        
        Ok(())
    }

    pub async fn upload_file(
        &self,
        file_data: &[u8],
        content_type: &str,
        folder: &str,
        original_filename: &str,
    ) -> Result<String> {
        self.ensure_bucket_exists().await?;

        // Validate file type based on content type
        Self::validate_file_type(content_type, file_data)?;

        // Generate unique filename
        let file_extension = Self::get_file_extension(original_filename);
        let unique_filename = format!("{}/{}.{}", folder, Uuid::new_v4(), file_extension);

        // Upload file
        let mut cursor = Cursor::new(file_data);
        let mut put_object_args = PutObjectArgs::new(
            &self.bucket_name,
            &unique_filename,
            &mut cursor,
            Some(file_data.len()),
            None, // No additional metadata size
        )?;

        self.client.put_object(&mut put_object_args).await?;

        Ok(unique_filename)
    }

    pub async fn upload_base64_file(
        &self,
        base64_data: &str,
        content_type: &str,
        folder: &str,
        original_filename: &str,
    ) -> Result<String> {
        // Remove data URL prefix if present
        let base64_clean = if base64_data.contains(',') {
            base64_data.split(',').nth(1).unwrap_or(base64_data)
        } else {
            base64_data
        };

        // Decode base64
        let file_data = general_purpose::STANDARD.decode(base64_clean)
            .map_err(|e| anyhow::anyhow!("Invalid base64 data: {}", e))?;

        self.upload_file(&file_data, content_type, folder, original_filename).await
    }

    pub async fn get_presigned_url(&self, object_name: &str, _expiry_seconds: u32) -> Result<String> {
        use http::Method;
        
        let get_presigned_args = GetPresignedObjectUrlArgs::new(
            &self.bucket_name,
            object_name,
            Method::GET,
        )?;

        let url = self.client.get_presigned_object_url(&get_presigned_args).await?;
        Ok(url.url)
    }

    pub async fn delete_file(&self, object_name: &str) -> Result<()> {
        use minio::s3::args::RemoveObjectArgs;
        let remove_args = RemoveObjectArgs::new(&self.bucket_name, object_name)?;
        self.client.remove_object(&remove_args).await?;
        Ok(())
    }

    fn validate_file_type(content_type: &str, file_data: &[u8]) -> Result<()> {
        // Validate file size (10MB max)
        const MAX_SIZE: usize = 10 * 1024 * 1024; // 10MB
        if file_data.len() > MAX_SIZE {
            bail!("File size exceeds 10MB limit");
        }

        // Validate content type and magic numbers
        match content_type {
            "image/jpeg" | "image/jpg" => {
                if !file_data.starts_with(&[0xFF, 0xD8, 0xFF]) {
                    bail!("Invalid JPEG file");
                }
            },
            "image/png" => {
                if !file_data.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
                    bail!("Invalid PNG file");
                }
            },
            "application/pdf" => {
                if !file_data.starts_with(b"%PDF") {
                    bail!("Invalid PDF file");
                }
            },
            _ => {
                bail!("Unsupported file type: {}", content_type);
            }
        }

        Ok(())
    }

    fn get_file_extension(filename: &str) -> String {
        std::path::Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("bin")
            .to_lowercase()
    }
}

#[derive(Debug, Clone)]
pub struct UploadResult {
    pub object_name: String,
    pub url: String,
    pub size: usize,
    pub content_type: String,
}

// Configuration struct for easier management
#[derive(Debug, Clone)]
pub struct MinioConfig {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket_name: String,
    pub secure: bool,
}

impl MinioConfig {
    pub fn from_env() -> Result<Self> {
        let endpoint = std::env::var("MINIO_ENDPOINT")
            .unwrap_or_else(|_| "localhost:9000".to_string());
        let access_key = std::env::var("MINIO_ACCESS_KEY")
            .map_err(|_| anyhow::anyhow!("MINIO_ACCESS_KEY environment variable not set"))?;
        let secret_key = std::env::var("MINIO_SECRET_KEY")
            .map_err(|_| anyhow::anyhow!("MINIO_SECRET_KEY environment variable not set"))?;
        let bucket_name = std::env::var("MINIO_BUCKET")
            .unwrap_or_else(|_| "imphnen-uploads".to_string());
        let secure = std::env::var("MINIO_SECURE")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false);

        Ok(Self {
            endpoint,
            access_key,
            secret_key,
            bucket_name,
            secure,
        })
    }

    pub fn endpoint_url(&self) -> String {
        if self.secure {
            format!("https://{}", self.endpoint)
        } else {
            format!("http://{}", self.endpoint)
        }
    }
}

// File type enumeration for better organization
#[derive(Debug, Clone)]
pub enum FileType {
    ProfileImage,
    CvResume,
    Document,
}

impl FileType {
    pub fn as_folder(&self) -> &str {
        match self {
            FileType::ProfileImage => "profiles",
            FileType::CvResume => "resumes",
            FileType::Document => "documents",
        }
    }

    pub fn max_size(&self) -> usize {
        match self {
            FileType::ProfileImage => 5 * 1024 * 1024,  // 5MB
            FileType::CvResume => 10 * 1024 * 1024,     // 10MB
            FileType::Document => 10 * 1024 * 1024,     // 10MB
        }
    }

    pub fn allowed_types(&self) -> Vec<&str> {
        match self {
            FileType::ProfileImage => vec!["image/jpeg", "image/png", "image/webp"],
            FileType::CvResume => vec!["application/pdf", "application/msword", "application/vnd.openxmlformats-officedocument.wordprocessingml.document"],
            FileType::Document => vec!["application/pdf", "image/jpeg", "image/png"],
        }
    }
}

// Upload request structure
#[derive(Debug, Clone)]
pub struct UploadRequest {
    pub user_id: String,
    pub file_type: FileType,
    pub filename: String,
    pub content_type: String,
    pub data: Vec<u8>,
}

// File metadata response
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub filename: String,
    pub content_type: String,
    pub size: usize,
    pub path: String,
    pub url: String,
}

// Helper function to decode base64 file data
pub fn decode_base64_file(base64_data: &str) -> Result<Vec<u8>> {
    // Remove data URL prefix if present (e.g., "data:image/jpeg;base64,")
    let clean_data = if base64_data.contains(',') {
        base64_data.split(',').nth(1).unwrap_or(base64_data)
    } else {
        base64_data
    };

    general_purpose::STANDARD
        .decode(clean_data)
        .map_err(|e| anyhow::anyhow!("Failed to decode base64 data: {}", e))
}

// Helper function to extract content type from data URL
pub fn extract_content_type_from_data_url(data_url: &str) -> Option<String> {
    if data_url.starts_with("data:") {
        if let Some(type_part) = data_url.split(';').next() {
            return Some(type_part.replace("data:", ""));
        }
    }
    None
}

// Helper function to create MinIO service from config
pub async fn create_minio_service_from_config(config: MinioConfig) -> Result<MinioService> {
    MinioService::new(
        &config.endpoint_url(),
        &config.access_key,
        &config.secret_key,
        &config.bucket_name,
    ).await
}
