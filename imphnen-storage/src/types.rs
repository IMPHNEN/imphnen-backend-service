use anyhow::{Result, bail};

#[derive(Debug, Clone)]
pub struct UploadResult {
	pub object_name: String,
	pub url: String,
	pub size: usize,
	pub content_type: String,
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
			FileType::Jpeg | FileType::Png | FileType::Webp | FileType::Gif => {
				5 * 1024 * 1024
			}
			FileType::Pdf | FileType::Doc | FileType::Docx => 10 * 1024 * 1024,
			FileType::Unknown => 5 * 1024 * 1024,
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
			FileType::Docx => vec![
				"application/vnd.openxmlformats-officedocument.wordprocessingml.document",
			],
			FileType::Unknown => vec![],
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
			"application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
				FileType::Docx
			}
			_ => FileType::Unknown,
		}
	}

	pub fn from_filename(filename: &str) -> Self {
		let f = filename.to_lowercase();
		if f.ends_with(".jpg") || f.ends_with(".jpeg") {
			FileType::Jpeg
		} else if f.ends_with(".png") {
			FileType::Png
		} else if f.ends_with(".webp") {
			FileType::Webp
		} else if f.ends_with(".gif") {
			FileType::Gif
		} else if f.ends_with(".pdf") {
			FileType::Pdf
		} else if f.ends_with(".doc") {
			FileType::Doc
		} else if f.ends_with(".docx") {
			FileType::Docx
		} else {
			FileType::Unknown
		}
	}
}

pub fn validate_file_type(content_type: &str, file_data: &[u8]) -> Result<()> {
	const MAX_SIZE: usize = 10 * 1024 * 1024;
	if file_data.len() > MAX_SIZE {
		bail!("File size exceeds 10MB limit");
	}
	match content_type {
		"image/jpeg" | "image/jpg" => {
			if !file_data.starts_with(&[0xFF, 0xD8, 0xFF]) {
				bail!("Invalid JPEG file");
			}
		}
		"image/png" => {
			if !file_data.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
				bail!("Invalid PNG file");
			}
		}
		"application/pdf" => {
			if !file_data.starts_with(b"%PDF") {
				bail!("Invalid PDF file");
			}
		}
		"image/webp" => {
			if !file_data.starts_with(b"RIFF")
				|| file_data.get(8..12).is_none_or(|s| s != b"WEBP")
			{
				bail!("Invalid WebP file");
			}
		}
		"application/msword"
		| "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
			if file_data.len() < 512 {
				bail!("Invalid document file");
			}
		}
		_ => bail!("Unsupported file type: {}", content_type),
	}
	Ok(())
}

pub fn get_file_extension(filename: &str) -> String {
	std::path::Path::new(filename)
		.extension()
		.and_then(|ext| ext.to_str())
		.unwrap_or("bin")
		.to_lowercase()
}
