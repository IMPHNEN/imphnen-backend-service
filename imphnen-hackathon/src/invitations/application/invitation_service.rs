use crate::invitations::domain::entity::*;
use crate::invitations::domain::repository::InvitationRepository;
use crate::invitations::domain::service::InvitationService;
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

pub struct InvitationServiceImpl {
	repo: Arc<dyn InvitationRepository>,
}

impl InvitationServiceImpl {
	pub fn new(repo: Arc<dyn InvitationRepository>) -> Self {
		Self { repo }
	}

	async fn do_invite(
		&self,
		team_id: Uuid,
		inviter_id: Uuid,
		input: CreateInvitationInput,
	) -> Result<InvitationWithDetails, AppError> {
		if is_team_features_closed() {
			return Err(AppError::BadRequestError(
				"Team invitations are closed (deadline: November 30, 2025).".to_string(),
			));
		}
		let leader_id = self
			.repo
			.get_team_leader_id(team_id)
			.await?
			.ok_or_else(|| AppError::NotFoundError("Team not found".to_string()))?;
		if leader_id != inviter_id {
			return Err(AppError::ForbiddenError(
				"Only the team leader can send invitations".to_string(),
			));
		}
		if self.repo.team_has_submission(team_id).await? {
			return Err(AppError::BadRequestError(
				"Cannot invite after submitting a project".to_string(),
			));
		}
		let count = self.repo.active_member_count(team_id).await?;
		if count >= 5 {
			return Err(AppError::BadRequestError(
				"Team already has the maximum of 5 members".to_string(),
			));
		}
		let team_name = self
			.repo
			.get_team_name(team_id)
			.await?
			.ok_or_else(|| AppError::NotFoundError("Team not found".to_string()))?;
		let inviter_fullname = self
			.repo
			.get_inviter_name(inviter_id)
			.await?
			.unwrap_or_else(|| "Unknown".to_string());
		let invitation_id = Uuid::new_v4();
		let entity = self
			.repo
			.create(invitation_id, team_id, inviter_id, &input.invitee_email)
			.await?;
		tracing::warn!(
			"Email sending is not available; invitation created for {}",
			input.invitee_email
		);
		Ok(InvitationWithDetails {
			id: entity.id,
			team_id: entity.team_id,
			team_name,
			inviter_id: entity.inviter_id,
			inviter_fullname,
			invitee_email: entity.invitee_email,
			status: entity.status,
			created_at: entity.created_at,
		})
	}
}

#[async_trait]
impl InvitationService for InvitationServiceImpl {
	async fn invite_member(
		&self,
		team_id: Uuid,
		inviter_id: Uuid,
		input: CreateInvitationInput,
	) -> Result<InvitationWithDetails, AppError> {
		self.do_invite(team_id, inviter_id, input).await
	}

	async fn invite_member_for_team(
		&self,
		team_id: Uuid,
		inviter_id: Uuid,
		input: CreateInvitationInput,
	) -> Result<InvitationWithDetails, AppError> {
		self.do_invite(team_id, inviter_id, input).await
	}

	async fn get_my_invitations(
		&self,
		user_id: Uuid,
	) -> Result<Vec<InvitationWithDetails>, AppError> {
		let email = self
			.repo
			.get_user_email(user_id)
			.await?
			.ok_or_else(|| AppError::NotFoundError("User not found".to_string()))?;
		self.repo.find_pending_by_email(&email).await
	}

	async fn respond_to_invitation(
		&self,
		invitation_id: Uuid,
		user_id: Uuid,
		accept: bool,
	) -> Result<(), AppError> {
		let invitation =
			self.repo.find_by_id(invitation_id).await?.ok_or_else(|| {
				AppError::NotFoundError("Invitation not found".to_string())
			})?;
		let user_email = self
			.repo
			.get_user_email(user_id)
			.await?
			.ok_or_else(|| AppError::NotFoundError("User not found".to_string()))?;
		if invitation.invitee_email != user_email {
			return Err(AppError::ForbiddenError(
				"This invitation is not for you".to_string(),
			));
		}
		if invitation.status != "pending" {
			return Err(AppError::BadRequestError(
				"Invitation is no longer pending".to_string(),
			));
		}
		if accept {
			if self.repo.team_has_submission(invitation.team_id).await? {
				return Err(AppError::BadRequestError(
					"Cannot join a team that has already submitted".to_string(),
				));
			}
			if let Some(active_team) = self.repo.user_active_team_name(user_id).await? {
				return Err(AppError::ConflictError(format!(
					"You are already a member of team '{}'",
					active_team
				)));
			}
			let count = self.repo.active_member_count(invitation.team_id).await?;
			if count >= 5 {
				return Err(AppError::BadRequestError(
					"Team is already full".to_string(),
				));
			}
			self.repo.update_status(invitation_id, "accepted").await?;
			self
				.repo
				.add_team_member(invitation.team_id, user_id)
				.await?;
			self
				.repo
				.reject_pending_for_email_except(&user_email, invitation_id)
				.await?;
			self
				.repo
				.reject_pending_join_requests_for_user(user_id)
				.await?;
		} else {
			self.repo.update_status(invitation_id, "rejected").await?;
		}
		Ok(())
	}
}
