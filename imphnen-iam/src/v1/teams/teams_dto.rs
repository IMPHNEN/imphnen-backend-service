use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct TeamsCreateRequestDto {
	#[validate(length(min = 3, max = 100, message = "Team name must be between 3 and 100 characters"))]
	pub name: String,
	
	#[validate(length(max = 500, message = "Description cannot exceed 500 characters"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_open: Option<bool>,
	
	#[validate(range(min = 2, max = 50, message = "Max members must be between 2 and 50"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_members: Option<i32>,
	
	#[serde(skip_serializing_if = "Option::is_none")]
	pub skills_required: Option<Vec<String>>,
	
	#[validate(length(max = 100, message = "Location cannot exceed 100 characters"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub location: Option<String>,
	
	#[validate(url(message = "Invalid avatar URL"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub avatar: Option<String>,
	
	#[validate(url(message = "Invalid website URL"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub website_url: Option<String>,
	
	#[validate(url(message = "Invalid GitHub URL"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub github_url: Option<String>,
	
	#[validate(length(min = 1, message = "Member emails cannot be empty"))]
	pub member_emails: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct TeamsUpdateRequestDto {
	#[validate(length(min = 3, max = 100, message = "Team name must be between 3 and 100 characters"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	
	#[validate(length(max = 500, message = "Description cannot exceed 500 characters"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_open: Option<bool>,
	
	#[validate(range(min = 2, max = 50, message = "Max members must be between 2 and 50"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_members: Option<i32>,
	
	#[serde(skip_serializing_if = "Option::is_none")]
	pub skills_required: Option<Vec<String>>,
	
	#[validate(length(max = 100, message = "Location cannot exceed 100 characters"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub location: Option<String>,
	
	#[validate(url(message = "Invalid avatar URL"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub avatar: Option<String>,
	
	#[validate(url(message = "Invalid website URL"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub website_url: Option<String>,
	
	#[validate(url(message = "Invalid GitHub URL"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub github_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct TeamInviteRequestDto {
	#[validate(length(min = 1, message = "Member emails cannot be empty"))]
	pub member_emails: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TeamAcceptInvitationRequestDto {
	pub token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TeamsDetailItemDto {
	pub id: String,
	pub name: String,
	pub description: Option<String>,
	pub leader: TeamMemberDto,
	pub is_open: bool,
	pub max_members: Option<i32>,
	pub current_member_count: i32,
	pub skills_required: Option<Vec<String>>,
	pub location: Option<String>,
	pub avatar: Option<String>,
	pub website_url: Option<String>,
	pub github_url: Option<String>,
	pub members: Option<Vec<TeamMemberDto>>,
	pub is_active: bool,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TeamsListItemDto {
	pub id: String,
	pub name: String,
	pub description: Option<String>,
	pub leader: TeamMemberDto,
	pub is_open: bool,
	pub current_member_count: i32,
	pub max_members: Option<i32>,
	pub skills_required: Option<Vec<String>>,
	pub location: Option<String>,
	pub avatar: Option<String>,
	pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TeamMemberDto {
	pub id: String,
	pub user_id: String,
	pub fullname: String,
	pub email: Option<String>,
	pub avatar: Option<String>,
	pub role: String,
	pub skills: Option<Vec<String>>,
	pub joined_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TeamInvitationDto {
	pub id: String,
	pub team_id: String,
	pub team_name: String,
	pub email: String,
	pub inviter_name: String,
	pub status: String,
	pub expires_at: String,
	pub invited_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TeamsDetailQueryDto {
	pub id: Thing,
	pub name: String,
	pub description: Option<String>,
	pub leader_id: Thing,
	pub is_open: bool,
	pub max_members: Option<i32>,
	pub skills_required: Option<Vec<String>>,
	pub location: Option<String>,
	pub avatar: Option<String>,
	pub website_url: Option<String>,
	pub github_url: Option<String>,
	pub is_active: bool,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TeamsListQueryDto {
	pub id: Thing,
	pub name: String,
	pub description: Option<String>,
	pub leader_id: Thing,
	pub is_open: bool,
	pub max_members: Option<i32>,
	pub skills_required: Option<Vec<String>>,
	pub location: Option<String>,
	pub avatar: Option<String>,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TeamMembersQueryDto {
	pub id: Thing,
	pub team_id: Thing,
	pub user_id: Thing,
	pub role: String,
	pub joined_at: String,
	pub is_active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TeamInvitationsQueryDto {
	pub id: Thing,
	pub team_id: Thing,
	pub email: String,
	pub inviter_id: Thing,
	pub invite_code: String,
	pub expires_at: String,
	pub status: String,
	pub invited_at: String,
	pub accepted_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TeamsSearchQueryDto {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub query: Option<String>,
	
	#[serde(skip_serializing_if = "Option::is_none")]
	pub open: Option<bool>,
	
	#[serde(skip_serializing_if = "Option::is_none")]
	pub skills: Option<Vec<String>>,
	
	#[serde(skip_serializing_if = "Option::is_none")]
	pub location: Option<String>,
	
	#[serde(skip_serializing_if = "Option::is_none")]
	pub page: Option<i64>,
	
	#[serde(skip_serializing_if = "Option::is_none")]
	pub per_page: Option<i64>,
}

impl TeamsDetailQueryDto {
	pub fn from(self) -> Self {
		self
	}
}

impl TeamsListQueryDto {
	pub fn from(self) -> TeamsListItemDto {
		TeamsListItemDto {
			id: self.id.id.to_raw(),
			name: self.name,
			description: self.description,
			leader: TeamMemberDto {
				id: String::new(),
				user_id: self.leader_id.id.to_raw(),
				fullname: String::new(),
				email: None,
				avatar: None,
				role: "leader".to_string(),
				skills: None,
				joined_at: self.created_at.clone(),
			},
			is_open: self.is_open,
			current_member_count: 1,
			max_members: self.max_members,
			skills_required: self.skills_required,
			location: self.location,
			avatar: self.avatar,
			created_at: self.created_at,
		}
	}
}