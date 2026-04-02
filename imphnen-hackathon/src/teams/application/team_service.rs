use std::sync::Arc;
use uuid::Uuid;
use async_trait::async_trait;
use chrono::{Utc, TimeZone};
use imphnen_utils::errors::AppError;
use crate::teams::domain::entity::*;
use crate::teams::domain::repository::TeamRepository;
use crate::teams::domain::service::TeamService;
use crate::common::cities::is_valid_indonesian_city;

fn is_team_features_closed() -> bool {
    let deadline = Utc.with_ymd_and_hms(2025, 11, 30, 16, 59, 0).unwrap();
    Utc::now() >= deadline
}

fn team_features_closed_err() -> AppError {
    AppError::BadRequestError("Team features are closed. The deadline was November 30, 2025 at 23:59 WIB.".to_string())
}

pub struct TeamServiceImpl {
    repo: Arc<dyn TeamRepository>,
}

impl TeamServiceImpl {
    pub fn new(repo: Arc<dyn TeamRepository>) -> Self { Self { repo } }

    async fn assemble_team_details(&self, entity: TeamEntity) -> Result<TeamWithDetails, AppError> {
        let leader = self.repo.get_leader(entity.leader_id).await?;
        let members = self.repo.get_members(entity.id).await?;
        let member_count = members.len() as i64;
        let has_submission = self.repo.team_has_submission(entity.id).await?;
        Ok(TeamWithDetails {
            id: entity.id,
            name: entity.name,
            description: entity.description,
            city: entity.city,
            visibility: entity.visibility,
            logo: entity.logo,
            banner: entity.banner,
            leader_id: entity.leader_id,
            leader,
            members: Some(members),
            member_count: Some(member_count),
            has_submission: Some(has_submission),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        })
    }
}

#[async_trait]
impl TeamService for TeamServiceImpl {
    async fn create_team(&self, user_id: Uuid, input: CreateTeamInput) -> Result<TeamWithDetails, AppError> {
        if is_team_features_closed() { return Err(team_features_closed_err()); }
        if !is_valid_indonesian_city(&input.city) {
            return Err(AppError::BadRequestError(format!("Invalid city '{}'. Only Indonesian cities are allowed.", input.city)));
        }
        if let Some(name) = self.repo.user_active_team_name(user_id).await? {
            return Err(AppError::ConflictError(format!("You are already a member of team '{}'. Leave your current team first.", name)));
        }
        let id = Uuid::new_v4();
        let entity = self.repo.create(id, user_id, input).await?;
        self.repo.add_member(entity.id, user_id, "leader").await?;
        self.repo.reject_pending_invitations_for_user(user_id).await?;
        self.repo.reject_pending_join_requests_for_user(user_id).await?;
        self.assemble_team_details(entity).await
    }

    async fn get_team_by_id(&self, team_id: Uuid) -> Result<TeamWithDetails, AppError> {
        let entity = self.repo.find_by_id(team_id).await?
            .ok_or_else(|| AppError::NotFoundError("Team not found".to_string()))?;
        self.assemble_team_details(entity).await
    }

    async fn browse_teams(&self, input: BrowseTeamsInput) -> Result<BrowseTeamsResult, AppError> {
        let page = if input.page < 1 { 1 } else { input.page };
        let per_page = if input.per_page < 1 { 10 } else if input.per_page > 100 { 100 } else { input.per_page };
        let normalized = BrowseTeamsInput { page, per_page, ..input };
        let (teams, total) = self.repo.browse(normalized).await?;

        let leader_ids: Vec<Uuid> = teams.iter().map(|t| t.leader_id).collect();
        let team_ids: Vec<Uuid> = teams.iter().map(|t| t.id).collect();

        let leaders = if !leader_ids.is_empty() { self.repo.get_leaders_batch(leader_ids).await? } else { vec![] };
        let counts = if !team_ids.is_empty() { self.repo.get_member_counts_batch(team_ids.clone()).await? } else { vec![] };
        let submitted_ids = if !team_ids.is_empty() { self.repo.get_submitted_team_ids(team_ids).await? } else { vec![] };

        let result_teams: Vec<TeamWithDetails> = teams.into_iter().map(|t| {
            let leader = leaders.iter().find(|l| l.id == t.leader_id).cloned();
            let member_count = counts.iter().find(|(id, _)| *id == t.id).map(|(_, c)| *c);
            let has_submission = submitted_ids.contains(&t.id);
            TeamWithDetails {
                id: t.id, name: t.name, description: t.description, city: t.city,
                visibility: t.visibility, logo: t.logo, banner: t.banner, leader_id: t.leader_id,
                leader, members: None, member_count, has_submission: Some(has_submission),
                created_at: t.created_at, updated_at: t.updated_at,
            }
        }).collect();

        Ok(BrowseTeamsResult { teams: result_teams, total, page, per_page })
    }

    async fn get_user_teams(&self, user_id: Uuid) -> Result<Vec<TeamWithDetails>, AppError> {
        let teams = self.repo.find_by_user(user_id).await?;
        let leader_ids: Vec<Uuid> = teams.iter().map(|t| t.leader_id).collect();
        let team_ids: Vec<Uuid> = teams.iter().map(|t| t.id).collect();
        let leaders = if !leader_ids.is_empty() { self.repo.get_leaders_batch(leader_ids).await? } else { vec![] };
        let counts = if !team_ids.is_empty() { self.repo.get_member_counts_batch(team_ids).await? } else { vec![] };
        Ok(teams.into_iter().map(|t| {
            let leader = leaders.iter().find(|l| l.id == t.leader_id).cloned();
            let member_count = counts.iter().find(|(id, _)| *id == t.id).map(|(_, c)| *c);
            TeamWithDetails {
                id: t.id, name: t.name, description: t.description, city: t.city,
                visibility: t.visibility, logo: t.logo, banner: t.banner, leader_id: t.leader_id,
                leader, members: None, member_count, has_submission: None,
                created_at: t.created_at, updated_at: t.updated_at,
            }
        }).collect())
    }

    async fn update_team(&self, team_id: Uuid, user_id: Uuid, input: UpdateTeamInput) -> Result<TeamWithDetails, AppError> {
        if is_team_features_closed() { return Err(team_features_closed_err()); }
        if !self.repo.is_leader(team_id, user_id).await? {
            return Err(AppError::ForbiddenError("Only team leader can perform this action".to_string()));
        }
        if let Some(ref city) = input.city {
            if !is_valid_indonesian_city(city) {
                return Err(AppError::BadRequestError(format!("Invalid city '{}'. Only Indonesian cities are allowed.", city)));
            }
        }
        let entity = self.repo.update(team_id, input).await?;
        self.assemble_team_details(entity).await
    }

    async fn remove_team_member(&self, team_id: Uuid, user_id: Uuid, member_id: Uuid) -> Result<(), AppError> {
        if is_team_features_closed() { return Err(team_features_closed_err()); }
        if !self.repo.is_leader(team_id, user_id).await? {
            return Err(AppError::ForbiddenError("Only team leader can perform this action".to_string()));
        }
        if member_id == user_id {
            return Err(AppError::BadRequestError("Team leader cannot remove themselves".to_string()));
        }
        if self.repo.team_has_submission(team_id).await? {
            return Err(AppError::ConflictError("Cannot remove members after project submission".to_string()));
        }
        self.repo.remove_member(team_id, member_id).await
    }

    async fn leave_team(&self, team_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        if is_team_features_closed() { return Err(team_features_closed_err()); }
        if !self.repo.is_member(team_id, user_id).await? {
            return Err(AppError::NotFoundError("You are not a member of this team".to_string()));
        }
        if self.repo.team_has_submission(team_id).await? {
            return Err(AppError::ConflictError("Cannot leave team after project submission".to_string()));
        }
        if self.repo.is_leader(team_id, user_id).await? {
            return Err(AppError::BadRequestError("Team leader cannot leave team. Transfer leadership or delete the team.".to_string()));
        }
        self.repo.remove_member(team_id, user_id).await
    }

    async fn delete_team(&self, team_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        if !self.repo.is_leader(team_id, user_id).await? {
            return Err(AppError::ForbiddenError("Only team leader can perform this action".to_string()));
        }
        let count = self.repo.get_member_count(team_id).await?;
        if count > 1 {
            return Err(AppError::ConflictError("Cannot delete team with other members. Remove all members first.".to_string()));
        }
        let deleted = self.repo.delete(team_id).await?;
        if !deleted {
            return Err(AppError::NotFoundError("Team not found".to_string()));
        }
        Ok(())
    }
}
