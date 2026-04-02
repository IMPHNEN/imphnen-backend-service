use crate::join_requests::domain::entity::*;
use crate::join_requests::domain::repository::JoinRequestRepository;
use crate::join_requests::domain::service::JoinRequestService;
use async_trait::async_trait;
use chrono::{TimeZone, Utc};
use imphnen_utils::errors::AppError;
use std::sync::Arc;
use uuid::Uuid;

fn is_team_features_closed() -> bool {
	let deadline = Utc
		.with_ymd_and_hms(2025, 11, 30, 16, 59, 0)
		.single()
		.expect("valid constant date");
	Utc::now() >= deadline
}

pub struct JoinRequestServiceImpl {
	repo: Arc<dyn JoinRequestRepository>,
}

impl JoinRequestServiceImpl {
	pub fn new(repo: Arc<dyn JoinRequestRepository>) -> Self {
		Self { repo }
	}
}

#[async_trait]
impl JoinRequestService for JoinRequestServiceImpl {
	async fn create_join_request(
		&self,
		team_id: Uuid,
		user_id: Uuid,
		input: CreateJoinRequestInput,
	) -> Result<JoinRequestWithDetails, AppError> {
		if is_team_features_closed() {
			return Err(AppError::BadRequestError(
				"Join requests are closed (deadline: November 30, 2025).".to_string(),
			));
		}
		if !self.repo.team_exists(team_id).await? {
			return Err(AppError::NotFoundError("Team not found".to_string()));
		}
		if self.repo.team_has_submission(team_id).await? {
			return Err(AppError::BadRequestError(
				"Cannot request to join a team that has already submitted".to_string(),
			));
		}
		if let Some(active_team) = self.repo.user_active_team_name(user_id).await? {
			return Err(AppError::ConflictError(format!(
				"You are already a member of team '{}'",
				active_team
			)));
		}
		let count = self.repo.active_member_count(team_id).await?;
		if count >= 5 {
			return Err(AppError::BadRequestError(
				"Team is already full (max 5 members)".to_string(),
			));
		}
		if self.repo.pending_request_exists(team_id, user_id).await? {
			return Err(AppError::ConflictError(
				"You already have a pending request for this team".to_string(),
			));
		}
		let id = Uuid::new_v4();
		let entity = self
			.repo
			.create(id, team_id, user_id, &input.message)
			.await?;
		let details: Vec<JoinRequestWithDetails> =
			self.repo.find_by_user(user_id).await?;
		details
			.into_iter()
			.find(|r| r.id == entity.id)
			.ok_or_else(|| {
				AppError::InternalServerError(
					"Failed to retrieve created join request".to_string(),
				)
			})
	}

	async fn get_my_join_requests(
		&self,
		user_id: Uuid,
	) -> Result<Vec<JoinRequestWithDetails>, AppError> {
		self.repo.find_by_user(user_id).await
	}

	async fn get_team_join_requests(
		&self,
		team_id: Uuid,
		user_id: Uuid,
	) -> Result<Vec<JoinRequestWithDetails>, AppError> {
		let leader_id = self
			.repo
			.get_team_leader_id(team_id)
			.await?
			.ok_or_else(|| AppError::NotFoundError("Team not found".to_string()))?;
		if leader_id != user_id {
			return Err(AppError::ForbiddenError(
				"Only the team leader can view join requests".to_string(),
			));
		}
		self.repo.find_pending_by_team(team_id).await
	}

	async fn respond_to_join_request(
		&self,
		request_id: Uuid,
		user_id: Uuid,
		accept: bool,
	) -> Result<(), AppError> {
		let request = self.repo.find_by_id(request_id).await?.ok_or_else(|| {
			AppError::NotFoundError("Join request not found".to_string())
		})?;
		let leader_id = self
			.repo
			.get_team_leader_id(request.team_id)
			.await?
			.ok_or_else(|| AppError::NotFoundError("Team not found".to_string()))?;
		if leader_id != user_id {
			return Err(AppError::ForbiddenError(
				"Only the team leader can respond to join requests".to_string(),
			));
		}
		if request.status != "pending" {
			return Err(AppError::BadRequestError(
				"Join request is no longer pending".to_string(),
			));
		}
		if accept {
			if is_team_features_closed() {
				return Err(AppError::BadRequestError(
					"Team features are now closed".to_string(),
				));
			}
			if self.repo.team_has_submission(request.team_id).await? {
				return Err(AppError::BadRequestError(
					"Cannot accept join request after submitting a project".to_string(),
				));
			}
			if let Some(active_team) =
				self.repo.user_active_team_name(request.user_id).await?
			{
				return Err(AppError::ConflictError(format!(
					"User is already a member of team '{}'",
					active_team
				)));
			}
			let count = self.repo.active_member_count(request.team_id).await?;
			if count >= 5 {
				return Err(AppError::BadRequestError(
					"Team is already full".to_string(),
				));
			}
			self.repo.update_status(request_id, "accepted").await?;
			self
				.repo
				.add_team_member(request.team_id, request.user_id)
				.await?;
			self
				.repo
				.reject_pending_invitations_for_user(request.user_id)
				.await?;
			self
				.repo
				.reject_other_pending_for_user(request.user_id, request_id)
				.await?;
		} else {
			self.repo.update_status(request_id, "rejected").await?;
		}
		Ok(())
	}
}
