use async_trait::async_trait;
use image::{DynamicImage, GenericImageView, ImageFormat, imageops};
use imphnen_utils::errors::AppError;
use qrcode::QrCode;
use std::io::Cursor;
use std::sync::Arc;
use uuid::Uuid;

use crate::qr::campaigns::domain::{
	entity::{CampaignEntity, CreateCampaignInput},
	repository::CampaignRepository,
	service::QrCampaignService,
};

pub struct QrCampaignServiceImpl {
	repo: Arc<dyn CampaignRepository>,
}

impl QrCampaignServiceImpl {
	pub fn new(repo: Arc<dyn CampaignRepository>) -> Self {
		Self { repo }
	}
}

#[async_trait]
impl QrCampaignService for QrCampaignServiceImpl {
	async fn create(
		&self,
		name: String,
		url: String,
		created_by: Uuid,
	) -> Result<CampaignEntity, AppError> {
		let qr = QrCode::new(url.as_bytes())
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		let qr_img = qr
			.render::<image::Luma<u8>>()
			.min_dimensions(256, 256)
			.build();
		let mut qr_bytes = Vec::new();
		DynamicImage::ImageLuma8(qr_img)
			.write_to(&mut Cursor::new(&mut qr_bytes), ImageFormat::Png)
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		let input = CreateCampaignInput {
			name,
			url,
			created_by,
			qr_code_data: qr_bytes,
		};
		self.repo.create(input).await
	}

	async fn list_all(&self) -> Result<Vec<CampaignEntity>, AppError> {
		self.repo.find_all().await
	}

	async fn get_active_qr_data(&self) -> Result<Option<Vec<u8>>, AppError> {
		self.repo.find_active_qr_data().await
	}

	async fn set_active(&self, id: Uuid) -> Result<CampaignEntity, AppError> {
		self.repo.set_active(id).await
	}

	async fn delete(&self, id: Uuid) -> Result<(), AppError> {
		self.repo.delete(id).await
	}

	async fn process_image(&self, image_bytes: Vec<u8>) -> Result<Vec<u8>, AppError> {
		let qr_data = self
			.repo
			.find_active_qr_data()
			.await?
			.ok_or_else(|| AppError::NotFoundError("No active campaign".to_string()))?;

		let img = image::load_from_memory(&image_bytes)
			.map_err(|_| AppError::BadRequestError("Invalid image format".to_string()))?;

		let qr_img = image::load_from_memory(&qr_data).map_err(|_| {
			AppError::InternalServerError("Failed to load QR data".to_string())
		})?;

		let (w, h) = img.dimensions();
		let qr_size = (std::cmp::min(w, h) / 5).max(100);

		let qr_resized =
			qr_img.resize_exact(qr_size, qr_size, imageops::FilterType::Nearest);

		let mut output = img.to_rgba8();
		let x = (w - qr_size - 10) as i64;
		let y = (h - qr_size - 10) as i64;
		imageops::overlay(&mut output, &qr_resized.to_rgba8(), x, y);

		let mut out_bytes = Vec::new();
		DynamicImage::ImageRgba8(output)
			.write_to(&mut Cursor::new(&mut out_bytes), ImageFormat::Png)
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		Ok(out_bytes)
	}
}
