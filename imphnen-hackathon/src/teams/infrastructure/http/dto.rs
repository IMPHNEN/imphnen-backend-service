use crate::teams::domain::entity::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserInfoResponse {
	pub id: Uuid,
	pub email: String,
	pub fullname: String,
	pub avatar: Option<String>,
	pub phone_number: Option<String>,
	pub location: Option<String>,
	pub bio: Option<String>,
	pub skills: Option<Vec<String>>,
	pub is_active: Option<bool>,
}

impl From<TeamUserInfo> for UserInfoResponse {
	fn from(u: TeamUserInfo) -> Self {
		Self {
			id: u.id,
			email: u.email,
			fullname: u.fullname,
			avatar: u.avatar,
			phone_number: u.phone_number,
			location: u.location,
			bio: u.bio,
			skills: u.skills,
			is_active: u.is_active,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TeamMemberResponse {
	pub id: Uuid,
	pub team_id: Uuid,
	pub user_id: Uuid,
	pub user: UserInfoResponse,
	pub role: String,
	pub status: String,
	pub joined_at: Option<DateTime<Utc>>,
}

impl From<TeamMemberEntity> for TeamMemberResponse {
	fn from(m: TeamMemberEntity) -> Self {
		Self {
			id: m.id,
			team_id: m.team_id,
			user_id: m.user_id,
			user: UserInfoResponse::from(m.user),
			role: m.role,
			status: m.status,
			joined_at: m.joined_at,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TeamResponse {
	pub id: Uuid,
	pub name: String,
	pub description: Option<String>,
	pub city: String,
	pub visibility: String,
	pub logo: Option<String>,
	pub banner: Option<String>,
	pub leader_id: Uuid,
	pub leader: Option<UserInfoResponse>,
	pub members: Option<Vec<TeamMemberResponse>>,
	pub member_count: Option<i64>,
	pub has_submission: Option<bool>,
	pub created_at: Option<DateTime<Utc>>,
	pub updated_at: Option<DateTime<Utc>>,
}

impl From<TeamWithDetails> for TeamResponse {
	fn from(t: TeamWithDetails) -> Self {
		Self {
			id: t.id,
			name: t.name,
			description: t.description,
			city: t.city,
			visibility: t.visibility,
			logo: t.logo,
			banner: t.banner,
			leader_id: t.leader_id,
			leader: t.leader.map(UserInfoResponse::from),
			members: t
				.members
				.map(|ms| ms.into_iter().map(TeamMemberResponse::from).collect()),
			member_count: t.member_count,
			has_submission: t.has_submission,
			created_at: t.created_at,
			updated_at: t.updated_at,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateTeamRequest {
	pub name: String,
	pub description: Option<String>,
	pub city: String,
	pub visibility: String,
	pub logo: Option<String>,
	pub banner: Option<String>,
}

impl From<CreateTeamRequest> for CreateTeamInput {
	fn from(r: CreateTeamRequest) -> Self {
		Self {
			name: r.name,
			description: r.description,
			city: r.city,
			visibility: r.visibility,
			logo: r.logo,
			banner: r.banner,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateTeamRequest {
	pub name: Option<String>,
	pub description: Option<String>,
	pub city: Option<String>,
	pub visibility: Option<String>,
	pub logo: Option<String>,
	pub banner: Option<String>,
}

impl From<UpdateTeamRequest> for UpdateTeamInput {
	fn from(r: UpdateTeamRequest) -> Self {
		Self {
			name: r.name,
			description: r.description,
			city: r.city,
			visibility: r.visibility,
			logo: r.logo,
			banner: r.banner,
		}
	}
}

#[derive(Debug, Deserialize, ToSchema, utoipa::IntoParams)]
pub struct BrowseTeamsQuery {
	pub search: Option<String>,
	pub city: Option<String>,
	pub min_members: Option<i64>,
	pub max_members: Option<i64>,
	pub has_submission: Option<bool>,
	#[serde(default = "default_page")]
	pub page: i64,
	#[serde(default = "default_per_page")]
	pub per_page: i64,
}

fn default_page() -> i64 {
	1
}
fn default_per_page() -> i64 {
	10
}

impl From<BrowseTeamsQuery> for BrowseTeamsInput {
	fn from(q: BrowseTeamsQuery) -> Self {
		Self {
			search: q.search,
			city: q.city,
			min_members: q.min_members,
			max_members: q.max_members,
			has_submission: q.has_submission,
			page: q.page,
			per_page: q.per_page,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TeamListResponse {
	pub data: Vec<TeamResponse>,
	pub total: i64,
	pub page: i64,
	pub per_page: i64,
}
