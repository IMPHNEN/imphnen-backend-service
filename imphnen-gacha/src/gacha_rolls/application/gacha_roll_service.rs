use crate::gacha_claims::domain::{GachaClaimEntity, GachaClaimRepository};
use crate::gacha_credits::domain::GachaCreditRepository;
use crate::gacha_rolls::domain::{
	GachaRollEntity, GachaRollRepository, GachaRollService,
};
use async_trait::async_trait;
use imphnen_utils::AppError;
use rand::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

pub struct GachaRollServiceImpl {
	roll_repo: Arc<dyn GachaRollRepository>,
	credit_repo: Arc<dyn GachaCreditRepository>,
	claim_repo: Arc<dyn GachaClaimRepository>,
}

impl GachaRollServiceImpl {
	pub fn new(
		roll_repo: Arc<dyn GachaRollRepository>,
		credit_repo: Arc<dyn GachaCreditRepository>,
		claim_repo: Arc<dyn GachaClaimRepository>,
	) -> Self {
		Self {
			roll_repo,
			credit_repo,
			claim_repo,
		}
	}

	fn roll_once(rolls: &[GachaRollEntity]) -> Option<GachaRollEntity> {
		let filtered: Vec<&GachaRollEntity> = rolls
			.iter()
			.filter(|r| !r.is_deleted && r.quantity > 0)
			.collect();

		if filtered.is_empty() {
			return None;
		}

		let total_weight: f64 = filtered
			.iter()
			.map(|r| f64::from(r.weight) * f64::from(r.quantity))
			.sum();

		if total_weight <= 0.0 {
			let mut rng = rand::rngs::ThreadRng::default();
			let index = rng.random_range(0..filtered.len());
			return Some(filtered[index].clone());
		}

		let mut rng = rand::rngs::ThreadRng::default();
		let random_value = rng.random_range(0.0..total_weight);

		let mut cumulative_weight = 0.0;
		for roll in &filtered {
			cumulative_weight += f64::from(roll.weight) * f64::from(roll.quantity);
			if random_value <= cumulative_weight {
				return Some((*roll).clone());
			}
		}

		Some(filtered[0].clone())
	}
}

#[async_trait]
impl GachaRollService for GachaRollServiceImpl {
	async fn get_roll(&self, id: Uuid) -> Result<GachaRollEntity, AppError> {
		self.roll_repo.find_by_id(id).await
	}

	async fn create_roll(&self, entity: GachaRollEntity) -> Result<(), AppError> {
		self.roll_repo.create(entity).await
	}

	async fn execute_roll(&self, user_id: Uuid) -> Result<GachaRollEntity, AppError> {
		let credit = self
			.credit_repo
			.find_by_user_id(user_id)
			.await?
			.ok_or_else(|| {
				AppError::BadRequestError("No credit record found".to_string())
			})?;

		if credit.available_rolls <= 0 {
			return Err(AppError::BadRequestError(
				"Not enough credits to perform this action".to_string(),
			));
		}

		self.credit_repo.consume_credit(user_id).await?;

		let rolls = self
			.roll_repo
			.find_all_active()
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		let selected = Self::roll_once(&rolls).ok_or_else(|| {
			AppError::NotFoundError("No rollable item available".to_string())
		});

		let selected = match selected {
			Ok(r) => r,
			Err(e) => {
				let _ = self.credit_repo.add_credit(user_id, 1).await;
				return Err(e);
			}
		};

		let claim_entity = GachaClaimEntity {
			id: Uuid::new_v4(),
			user_id,
			gacha_item_id: selected.item_id,
			claim_id: Uuid::new_v4(),
			claim_type: "roll".to_string(),
			status: "claimed".to_string(),
			quantity: 1,
			metadata: None,
			is_deleted: false,
			claimed_at: chrono::Utc::now(),
			created_at: chrono::Utc::now(),
			updated_at: chrono::Utc::now(),
			deleted_at: None,
		};

		if let Err(e) = self.claim_repo.create(claim_entity).await {
			let _ = self.credit_repo.add_credit(user_id, 1).await;
			return Err(e);
		}

		Ok(selected)
	}

	async fn delete_roll(&self, id: Uuid) -> Result<(), AppError> {
		self.roll_repo.delete(id).await
	}
}
