use anyhow::{anyhow, bail, Result};
use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use uuid::Uuid;
use crate::enviroment::ENV;

// CATATAN: Pastikan untuk menambahkan dependensi ini ke `Cargo.toml` Anda:
// reqwest = { version = "0.11", features = ["json"] }
// anyhow = "1.0"
// uuid = { version = "1.3", features = ["v4"] }
// base64 = "0.21"
// log = "0.4"
// chrono = "0.4"
// sha2 = "0.10"
// hmac = "0.12"
// hex = "0.4"

// --- Struct Konfigurasi MinIO ---
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
    /// Memuat konfigurasi MinIO dari variabel lingkungan.
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

    /// Mendapatkan URL endpoint lengkap (http atau https).
    pub fn endpoint_url(&self) -> String {
        // If endpoint already has protocol, use it as-is
        if self.endpoint.starts_with("http://") || self.endpoint.starts_with("https://") {
            self.endpoint.clone()
        } else {
            // Only add protocol if not present
            let protocol = if self.secure { "https" } else { "http" };
            format!("{}://{}", protocol, self.endpoint)
        }
    }
}

// --- Layanan MinIO ---
pub struct MinioService {
    endpoint: String,
    access_key: String,
    secret_key: String,
    bucket_name: String,
    region: String,
    client: reqwest::Client,
}

impl MinioService {
    /// Membuat instance layanan MinIO baru.
    pub async fn new(
        endpoint: &str,
        access_key: &str,
        secret_key: &str,
        bucket_name: &str,
        region: &str,
    ) -> Result<Self> {
        let service = Self {
            endpoint: endpoint.to_string(),
            access_key: access_key.to_string(),
            secret_key: secret_key.to_string(),
            bucket_name: bucket_name.to_string(),
            region: region.to_string(),
            client: reqwest::Client::new(),
        };
        Ok(service)
    }

    /// Mengunggah file biner ke MinIO.
    pub async fn upload_file(
        &self,
        file_data: &[u8],
        content_type: &str,
        folder: &str,
        original_filename: &str,
    ) -> Result<String> {
        Self::validate_file_type(content_type, file_data)?;

        let file_extension = Self::get_file_extension(original_filename);
        let unique_filename = format!("{}/{}.{}", folder, Uuid::new_v4(), file_extension);
        let object_name = &unique_filename;

        // Extract host from endpoint (remove protocol)
        let host = self.endpoint
            .trim_start_matches("https://")
            .trim_start_matches("http://");
        
        let url = format!("https://{}/{}/{}", host, self.bucket_name, object_name);

        // Debug logging
        log::debug!("MinIO Endpoint config: {}", self.endpoint);
        log::debug!("MinIO Region config: {}", self.region);
        log::debug!("MinIO Access Key: {}", self.access_key);
        log::debug!("MinIO Bucket: {}", self.bucket_name);
        log::debug!("Extracted host: {}", host);
        log::debug!("Final URL: {}", url);

        let now = Utc::now();
        let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();
        let date_stamp = now.format("%Y%m%d").to_string();
        
        // 1) Use UNSIGNED-PAYLOAD for HTTPS uploads (safer for proxies)
        let payload_hash = "UNSIGNED-PAYLOAD".to_string();

        // 2) Path-style canonical URI: /{bucket}/{object}
        let canonical_uri = format!("/{}/{}", self.bucket_name, object_name);

        // 3) ONLY sign essential headers (no content-type to avoid proxy issues)
        let canonical_headers = format!(
            "host:{}\nx-amz-content-sha256:{}\nx-amz-date:{}\n",
            host, payload_hash, amz_date
        );
        let signed_headers = "host;x-amz-content-sha256;x-amz-date";
        
        // 4) Canonical request
        let canonical_request = format!(
            "PUT\n{}\n\n{}\n{}\n{}",
            canonical_uri, canonical_headers, signed_headers, payload_hash
        );

        // Debug logging
        log::debug!("URL: {}", url);
        log::debug!("Host: {}", host);
        log::debug!("Bucket: {}", self.bucket_name);
        log::debug!("Object: {}", object_name);
        log::debug!("Canonical URI: {}", canonical_uri);
        log::debug!("Payload hash: {}", payload_hash);
        log::debug!("Canonical Request:\n{}", canonical_request);

        let scope = format!("{}/{}/s3/aws4_request", date_stamp, self.region);
        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            amz_date,
            scope,
            hex::encode(Sha256::digest(canonical_request.as_bytes()))
        );

        log::debug!("Scope: {}", scope);
        log::debug!("String to sign:\n{}", string_to_sign);

        let signing_key = self.get_signature_key(&date_stamp)?;
        let mut mac = Hmac::<Sha256>::new_from_slice(&signing_key)?;
        mac.update(string_to_sign.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        log::debug!("Generated signature: {}", signature);

        let auth_header = format!(
            "AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
            self.access_key, scope, signed_headers, signature
        );

        // 5) Send request: Content-Type included but NOT signed
        let response = self
            .client
            .put(&url)
            .header("x-amz-date", &amz_date)
            .header("x-amz-content-sha256", &payload_hash)
            .header("Authorization", &auth_header)
            .header("Content-Type", content_type)
            // Don't set Host header manually - let reqwest handle it
            // Add headers for reverse proxy support (not signed)
            .header("X-Forwarded-Proto", "https")
            .header("X-Forwarded-Host", host)
            .body(file_data.to_vec())
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await?;
            bail!(
                "Gagal mengunggah file ke MinIO. Status: {}. Pesan: {}",
                status,
                error_body
            );
        }

        log::info!("Unggahan berhasil: {} byte ke {}", file_data.len(), unique_filename);
        Ok(unique_filename)
    }

    /// Mengunggah file yang dikodekan base64 ke MinIO.
    pub async fn upload_base64_file(
        &self,
        base64_data: &str,
        content_type: &str,
        folder: &str,
        original_filename: &str,
    ) -> Result<String> {
        let file_data = decode_base64_file(base64_data)?;
        self.upload_file(&file_data, content_type, folder, original_filename)
            .await
    }

    /// Menghasilkan URL yang telah ditandatangani sebelumnya untuk mengunduh objek.
    pub async fn get_presigned_url(&self, object_name: &str, expiry_seconds: u32) -> Result<String> {
        // Extract host from endpoint (remove protocol)
        let host = self.endpoint
            .trim_start_matches("https://")
            .trim_start_matches("http://");
            
        let now = Utc::now();
        let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();
        let date_stamp = now.format("%Y%m%d").to_string();
        let scope = format!("{}/{}/s3/aws4_request", date_stamp, self.region);
        let credential = format!("{}/{}", self.access_key, scope);

        let expires_str = expiry_seconds.to_string();
        let mut query_params = std::collections::BTreeMap::new();
        query_params.insert("X-Amz-Algorithm", "AWS4-HMAC-SHA256");
        query_params.insert("X-Amz-Credential", &credential);
        query_params.insert("X-Amz-Date", &amz_date);
        query_params.insert("X-Amz-Expires", &expires_str);
        query_params.insert("X-Amz-SignedHeaders", "host");

        let canonical_query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let canonical_request = format!(
            "GET\n/{}/{}\n{}\nhost:{}\n\nhost\nUNSIGNED-PAYLOAD",
            self.bucket_name, object_name, canonical_query_string, host
        );

        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            amz_date,
            scope,
            hex::encode(Sha256::digest(canonical_request.as_bytes()))
        );

        let signing_key = self.get_signature_key(&date_stamp)?;
        let mut mac = Hmac::<Sha256>::new_from_slice(&signing_key)?;
        mac.update(string_to_sign.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        let url = format!(
            "https://{}/{}/{}?{}&X-Amz-Signature={}",
            host, self.bucket_name, object_name, canonical_query_string, signature
        );

        Ok(url)
    }
    
    /// Menghapus file dari MinIO.
    pub async fn delete_file(&self, object_name: &str) -> Result<()> {
        // Extract host from endpoint (remove protocol)
        let host = self.endpoint
            .trim_start_matches("https://")
            .trim_start_matches("http://");
            
        let url = format!("https://{}/{}/{}", host, self.bucket_name, object_name);

        let now = Utc::now();
        let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();
        let date_stamp = now.format("%Y%m%d").to_string();
        let payload_hash = hex::encode(Sha256::digest(b""));

        let canonical_headers = format!("host:{}\nx-amz-content-sha256:{}\nx-amz-date:{}\n", host, payload_hash, amz_date);
        let signed_headers = "host;x-amz-content-sha256;x-amz-date";
        let canonical_request = format!(
            "DELETE\n/{}/{}\n\n{}\n{}\n{}",
            self.bucket_name, object_name, canonical_headers, signed_headers, payload_hash
        );

        let scope = format!("{}/{}/s3/aws4_request", date_stamp, self.region);
        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            amz_date,
            scope,
            hex::encode(Sha256::digest(canonical_request.as_bytes()))
        );

        // Debug logging for signature calculation
        log::debug!("Region: {}", self.region);
        log::debug!("Scope: {}", scope);
        log::debug!("String to sign:\n{}", string_to_sign);

        let signing_key = self.get_signature_key(&date_stamp)?;
        let mut mac = Hmac::<Sha256>::new_from_slice(&signing_key)?;
        mac.update(string_to_sign.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        log::debug!("Final signature: {}", signature);

        let auth_header = format!(
            "AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
            self.access_key, scope, signed_headers, signature
        );

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
                "Gagal menghapus file dari MinIO. Status: {}. Pesan: {}",
                status,
                error_body
            );
        }
        
        log::info!("File berhasil dihapus: {}", object_name);
        Ok(())
    }

    /// Fungsi pembantu untuk menghasilkan kunci tanda tangan AWS v4.
    fn get_signature_key(&self, date_stamp: &str) -> Result<Vec<u8>> {
        let secret = format!("AWS4{}", self.secret_key);
        let mut mac1 = Hmac::<Sha256>::new_from_slice(secret.as_bytes())?;
        mac1.update(date_stamp.as_bytes());
        let date_key = mac1.finalize().into_bytes();

        let mut mac2 = Hmac::<Sha256>::new_from_slice(&date_key)?;
        mac2.update(self.region.as_bytes());
        let date_region_key = mac2.finalize().into_bytes();

        let mut mac3 = Hmac::<Sha256>::new_from_slice(&date_region_key)?;
        mac3.update(b"s3");
        let date_region_service_key = mac3.finalize().into_bytes();

        let mut mac4 = Hmac::<Sha256>::new_from_slice(&date_region_service_key)?;
        mac4.update(b"aws4_request");
        Ok(mac4.finalize().into_bytes().to_vec())
    }

    /// Memvalidasi jenis file dan ukuran.
    fn validate_file_type(content_type: &str, file_data: &[u8]) -> Result<()> {
        const MAX_SIZE: usize = 10 * 1024 * 1024; // 10MB
        if file_data.len() > MAX_SIZE {
            bail!("Ukuran file melebihi batas 10MB");
        }

        match content_type {
            "image/jpeg" | "image/jpg" => {
                if !file_data.starts_with(&[0xFF, 0xD8, 0xFF]) {
                    bail!("File JPEG tidak valid");
                }
            }
            "image/png" => {
                if !file_data.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
                    bail!("File PNG tidak valid");
                }
            }
            "application/pdf" => {
                if !file_data.starts_with(b"%PDF") {
                    bail!("File PDF tidak valid");
                }
            }
            "image/webp" => {
                if !file_data.starts_with(b"RIFF")
                    || !file_data.get(8..12).map_or(false, |s| s == b"WEBP")
                {
                    bail!("File WEBP tidak valid");
                }
            }
            "application/msword" | "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
                if file_data.len() < 512 {
                    bail!("File dokumen tidak valid");
                }
            }
            _ => {
                bail!("Jenis file tidak didukung: {}", content_type);
            }
        }

        Ok(())
    }

    /// Mendapatkan ekstensi file dari nama file.
    fn get_file_extension(filename: &str) -> String {
        std::path::Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("bin")
            .to_lowercase()
    }
}

// --- Struct dan Enum Pembantu ---

#[derive(Debug, Clone)]
pub struct UploadResult {
    pub object_name: String,
    pub url: String,
    pub size: usize,
    pub content_type: String,
}

#[derive(Debug, Clone)]
pub enum FileType {
    Jpeg,
    Png,
    Webp,
    Gif,
    Pdf,
    Doc,
    Docx,
    Unknown,
}

impl FileType {
    pub fn as_folder(&self) -> &str {
        match self {
            FileType::Jpeg | FileType::Png | FileType::Webp | FileType::Gif => "profiles",
            FileType::Pdf | FileType::Doc | FileType::Docx => "documents",
            FileType::Unknown => "misc",
        }
    }

    pub fn max_size(&self) -> usize {
        match self {
            FileType::Jpeg | FileType::Png | FileType::Webp | FileType::Gif => 5 * 1024 * 1024,   // 5MB for images
            FileType::Pdf | FileType::Doc | FileType::Docx => 10 * 1024 * 1024,     // 10MB for documents
            FileType::Unknown => 5 * 1024 * 1024,      // 5MB default
        }
    }

    pub fn allowed_types(&self) -> Vec<&str> {
        match self {
            FileType::Jpeg => vec!["image/jpeg", "image/jpg"],
            FileType::Png => vec!["image/png"],
            FileType::Webp => vec!["image/webp"],
            FileType::Gif => vec!["image/gif"],
            FileType::Pdf => vec!["application/pdf"],
            FileType::Doc => vec!["application/msword"],
            FileType::Docx => vec!["application/vnd.openxmlformats-officedocument.wordprocessingml.document"],
            FileType::Unknown => vec![], // No allowed types for unknown
        }
    }
    
    pub fn from_content_type(content_type: &str) -> Self {
        match content_type {
            "image/jpeg" | "image/jpg" => FileType::Jpeg,
            "image/png" => FileType::Png,
            "image/webp" => FileType::Webp,
            "image/gif" => FileType::Gif,
            "application/pdf" => FileType::Pdf,
            "application/msword" => FileType::Doc,
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => FileType::Docx,
            _ => FileType::Unknown,
        }
    }
    
    pub fn from_filename(filename: &str) -> Self {
        let filename_lower = filename.to_lowercase();
        if filename_lower.ends_with(".jpg") || filename_lower.ends_with(".jpeg") {
            FileType::Jpeg
        } else if filename_lower.ends_with(".png") {
            FileType::Png
        } else if filename_lower.ends_with(".webp") {
            FileType::Webp
        } else if filename_lower.ends_with(".gif") {
            FileType::Gif
        } else if filename_lower.ends_with(".pdf") {
            FileType::Pdf
        } else if filename_lower.ends_with(".doc") {
            FileType::Doc
        } else if filename_lower.ends_with(".docx") {
            FileType::Docx
        } else {
            FileType::Unknown
        }
    }
}

#[derive(Debug, Clone)]
pub struct UploadRequest {
    pub user_id: String,
    pub file_type: FileType,
    pub filename: String,
    pub content_type: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub filename: String,
    pub content_type: String,
    pub size: usize,
    pub path: String,
    pub url: String,
}

// --- Fungsi Pembantu ---

/// Membuat instance MinioService dari struct MinioConfig.
pub async fn create_minio_service_from_config(config: MinioConfig) -> Result<MinioService> {
    MinioService::new(
        &config.endpoint,  // Use raw endpoint, not endpoint_url()
        &config.access_key,
        &config.secret_key,
        &config.bucket_name,
        &config.region,
    )
    .await
}

/// Mendekode data file base64.
pub fn decode_base64_file(base64_data: &str) -> Result<Vec<u8>> {
    let clean_data = if base64_data.contains(',') {
        base64_data.split(',').nth(1).unwrap_or(base64_data)
    } else {
        base64_data
    };

    general_purpose::STANDARD
        .decode(clean_data)
        .map_err(|e| anyhow!("Gagal mendekode data base64: {}", e))
}

/// Mengekstrak tipe konten dari URL data.
pub fn extract_content_type_from_data_url(data_url: &str) -> Option<String> {
    if data_url.starts_with("data:") {
        if let Some(type_part) = data_url.split(';').next() {
            return Some(type_part.replace("data:", ""));
        }
    }
    None
}
