use crate::sessions::domain::{
	AvailabilitySlot, MentorAvailability, SessionDetail, SessionList, SessionListItem,
	SessionRepository,
};
use chrono::{Duration, Utc};
use imphnen_utils::AppError;
use std::sync::Arc;
use uuid::Uuid;

pub struct SessionQueryService {
	pub repo: Arc<dyn SessionRepository>,
}

impl SessionQueryService {
	pub async fn get_mentor_sessions(
		&self,
		mentor_id: String,
		status_filter: Option<String>,
	) -> Result<SessionList, AppError> {
		let mentor_uuid = Uuid::parse_str(&mentor_id)
			.map_err(|e| AppError::BadRequestError(format!("Invalid mentor ID: {}", e)))?;

		let count = self
			.repo
			.count_by_mentor(mentor_uuid, status_filter.clone())
			.await?;

		let sessions = self
			.repo
			.find_by_mentor_id(mentor_uuid, status_filter)
			.await?;

		let items: Vec<SessionListItem> = sessions
			.into_iter()
			.map(|s| SessionListItem {
				id: s.id.to_string(),
				mentor_id: s.mentor_id.to_string(),
				mentee_id: s.mentee_id.to_string(),
				mentee_fullname: None,
				mentee_email: None,
				topic: s.topic,
				scheduled_at: s.scheduled_at.to_rfc3339(),
				duration_minutes: s.duration_minutes,
				session_type: s.session_type,
				status: s.status,
				rating: s.rating,
				created_at: s.created_at.to_rfc3339(),
			})
			.collect();

		Ok(SessionList {
			sessions: items,
			total: count,
		})
	}

	pub async fn get_user_sessions(
		&self,
		user_id: String,
		status_filter: Option<String>,
	) -> Result<SessionList, AppError> {
		let user_uuid = Uuid::parse_str(&user_id)
			.map_err(|e| AppError::BadRequestError(format!("Invalid user ID: {}", e)))?;

		let count = self
			.repo
			.count_by_mentee(user_uuid, status_filter.clone())
			.await?;

		let sessions = self
			.repo
			.find_by_mentee_id(user_uuid, status_filter)
			.await?;

		let items: Vec<SessionListItem> = sessions
			.into_iter()
			.map(|s| SessionListItem {
				id: s.id.to_string(),
				mentor_id: s.mentor_id.to_string(),
				mentee_id: s.mentee_id.to_string(),
				mentee_fullname: None,
				mentee_email: None,
				topic: s.topic,
				scheduled_at: s.scheduled_at.to_rfc3339(),
				duration_minutes: s.duration_minutes,
				session_type: s.session_type,
				status: s.status,
				rating: s.rating,
				created_at: s.created_at.to_rfc3339(),
			})
			.collect();

		Ok(SessionList {
			sessions: items,
			total: count,
		})
	}

	pub async fn get_mentor_availability(
		&self,
		mentor_id: String,
	) -> Result<MentorAvailability, AppError> {
		let mentor_uuid = Uuid::parse_str(&mentor_id)
			.map_err(|e| AppError::BadRequestError(format!("Invalid mentor ID: {}", e)))?;

		let booked_dates = self.repo.find_booked_dates(mentor_uuid).await?;

		let mut slots = Vec::new();
		let today = Utc::now().date_naive();

		for i in 0..7 {
			let date = today + Duration::days(i);
			let date_str = date.format("%Y-%m-%d").to_string();

			for hour in 9..17 {
				let time_str = format!("{:02}:00", hour);
				let datetime_prefix = format!("{}T{}", date_str, time_str);

				let is_booked = booked_dates
					.iter()
					.any(|d| d.starts_with(&datetime_prefix[..13]));

				slots.push(AvailabilitySlot {
					date: date_str.clone(),
					time: time_str,
					available: !is_booked,
				});
			}
		}

		Ok(MentorAvailability {
			mentor_id,
			availability_commitment: "Available weekdays 9 AM - 5 PM".to_string(),
			preferred_formats: vec!["video_call".to_string(), "phone_call".to_string()],
			slots,
			booked_dates,
		})
	}

	pub async fn get_session_detail(
		&self,
		session_id: String,
	) -> Result<SessionDetail, AppError> {
		let session_uuid = Uuid::parse_str(&session_id).map_err(|e| {
			AppError::BadRequestError(format!("Invalid session ID: {}", e))
		})?;

		let session = self
			.repo
			.find_by_id(session_uuid)
			.await?
			.ok_or_else(|| AppError::NotFoundError("Session not found".to_string()))?;

		Ok(SessionDetail {
			id: session.id.to_string(),
			mentor_id: session.mentor_id.to_string(),
			mentor_fullname: None,
			mentee_id: session.mentee_id.to_string(),
			mentee_fullname: None,
			topic: session.topic,
			description: session.description,
			scheduled_at: session.scheduled_at.to_rfc3339(),
			duration_minutes: session.duration_minutes,
			meeting_link: session.meeting_link,
			session_type: session.session_type,
			status: session.status,
			feedback: session.feedback,
			rating: session.rating,
			feedback_submitted_at: session.feedback_submitted_at.map(|dt| dt.to_rfc3339()),
			created_at: session.created_at.to_rfc3339(),
			updated_at: session.updated_at.to_rfc3339(),
		})
	}
}
