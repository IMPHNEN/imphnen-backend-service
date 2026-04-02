use super::entity::*;
use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use uuid::Uuid;

#[async_trait]
pub trait SubmissionService: Send + Sync {
	async fn create_submission(
		&self,
		team_id: Uuid,
		user_id: Uuid,
		input: CreateSubmissionInput,
	) -> Result<SubmissionEntity, AppError>;
	async fn get_team_submission(
		&self,
		team_id: Uuid,
		user_id: Uuid,
	) -> Result<SubmissionEntity, AppError>;
	async fn update_submission(
		&self,
		submission_id: Uuid,
		user_id: Uuid,
		input: UpdateSubmissionInput,
	) -> Result<SubmissionEntity, AppError>;
	async fn submit_project(
		&self,
		submission_id: Uuid,
		user_id: Uuid,
	) -> Result<SubmissionEntity, AppError>;
	async fn confirm_submission(
		&self,
		submission_id: Uuid,
		user_id: Uuid,
	) -> Result<SubmissionEntity, AppError>;
	async fn cancel_submission(
		&self,
		submission_id: Uuid,
		user_id: Uuid,
	) -> Result<SubmissionEntity, AppError>;
}
