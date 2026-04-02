use std::sync::Arc;
use uuid::Uuid;
use async_trait::async_trait;
use chrono::{Utc, TimeZone};
use imphnen_utils::errors::AppError;
use crate::submissions::domain::entity::*;
use crate::submissions::domain::repository::SubmissionRepository;
use crate::submissions::domain::service::SubmissionService;

fn is_submission_deadline_passed() -> bool {
    let deadline = Utc.with_ymd_and_hms(2025, 12, 7, 16, 59, 0).unwrap();
    Utc::now() >= deadline
}

pub struct SubmissionServiceImpl {
    repo: Arc<dyn SubmissionRepository>,
}

impl SubmissionServiceImpl {
    pub fn new(repo: Arc<dyn SubmissionRepository>) -> Self { Self { repo } }
}

#[async_trait]
impl SubmissionService for SubmissionServiceImpl {
    async fn create_submission(&self, team_id: Uuid, user_id: Uuid, input: CreateSubmissionInput) -> Result<SubmissionEntity, AppError> {
        if is_submission_deadline_passed() {
            return Err(AppError::BadRequestError("Submission deadline has passed (December 7, 2025 23:59 WIB).".to_string()));
        }
        if !self.repo.is_team_leader(team_id, user_id).await? {
            return Err(AppError::ForbiddenError("Only team leader can create submission".to_string()));
        }
        if self.repo.find_by_team(team_id).await?.is_some() {
            return Err(AppError::ConflictError("Team already has a submission".to_string()));
        }
        self.repo.create(team_id, user_id, input).await
    }

    async fn get_team_submission(&self, team_id: Uuid, user_id: Uuid) -> Result<SubmissionEntity, AppError> {
        if !self.repo.is_team_member(team_id, user_id).await? {
            return Err(AppError::ForbiddenError("Only team members can view submission".to_string()));
        }
        self.repo.find_by_team(team_id).await?.ok_or_else(|| AppError::NotFoundError("No submission found".to_string()))
    }

    async fn update_submission(&self, submission_id: Uuid, user_id: Uuid, input: UpdateSubmissionInput) -> Result<SubmissionEntity, AppError> {
        if is_submission_deadline_passed() {
            return Err(AppError::BadRequestError("Submission deadline has passed.".to_string()));
        }
        let sub = self.repo.find_by_id(submission_id).await?;
        if !self.repo.is_team_leader(sub.team_id, user_id).await? {
            return Err(AppError::ForbiddenError("Only team leader can update submission".to_string()));
        }
        if sub.status != "draft" {
            return Err(AppError::BadRequestError("Can only update draft submissions".to_string()));
        }
        self.repo.update(submission_id, input).await
    }

    async fn submit_project(&self, submission_id: Uuid, user_id: Uuid) -> Result<SubmissionEntity, AppError> {
        if is_submission_deadline_passed() {
            return Err(AppError::BadRequestError("Submission deadline has passed.".to_string()));
        }
        let sub = self.repo.find_by_id(submission_id).await?;
        if !self.repo.is_team_leader(sub.team_id, user_id).await? {
            return Err(AppError::ForbiddenError("Only team leader can submit".to_string()));
        }
        if sub.status != "draft" {
            return Err(AppError::BadRequestError("Can only submit from draft status".to_string()));
        }
        let count = self.repo.team_member_count(sub.team_id).await?;
        if count < 2 {
            return Err(AppError::BadRequestError("Team must have at least 2 members to submit".to_string()));
        }
        self.repo.update_status(submission_id, "pending").await
    }

    async fn confirm_submission(&self, submission_id: Uuid, user_id: Uuid) -> Result<SubmissionEntity, AppError> {
        let sub = self.repo.find_by_id(submission_id).await?;
        if !self.repo.is_team_leader(sub.team_id, user_id).await? {
            return Err(AppError::ForbiddenError("Only team leader can confirm submission".to_string()));
        }
        if sub.status != "pending" {
            return Err(AppError::BadRequestError("Can only confirm pending submissions".to_string()));
        }
        self.repo.update_status(submission_id, "submitted").await
    }

    async fn cancel_submission(&self, submission_id: Uuid, user_id: Uuid) -> Result<SubmissionEntity, AppError> {
        let sub = self.repo.find_by_id(submission_id).await?;
        if !self.repo.is_team_leader(sub.team_id, user_id).await? {
            return Err(AppError::ForbiddenError("Only team leader can cancel submission".to_string()));
        }
        if sub.status == "submitted" {
            return Err(AppError::BadRequestError("Cannot cancel a confirmed submission".to_string()));
        }
        self.repo.update_status(submission_id, "draft").await
    }
}
