use super::hackathon_schema::{HackathonSchema, HackathonStatus, HackathonTimelineSchema, HackathonPhase};
use anyhow::{Result, bail};
use std::collections::HashSet;
use chrono::Utc;

/// Validation rules for hackathon operations

// Constants for limits
pub const MAX_ORGANIZERS: usize = 20;
pub const MAX_TIMELINES: usize = 10;
pub const MAX_EVENTS_PER_HACKATHON: usize = 50;
pub const MAX_PRIZES: usize = 20;
pub const MIN_TEAM_SIZE: u32 = 2;
pub const MAX_TEAM_SIZE: u32 = 10;

/// Validate status transition
pub fn can_transition_status(
    current: &HackathonStatus,
    next: &HackathonStatus,
) -> Result<()> {
    use HackathonStatus::*;
    
    let allowed_transitions: Vec<HackathonStatus> = match current {
        Draft => vec![RegistrationOpen, Cancelled],
        RegistrationOpen => vec![RegistrationClosed, Cancelled],
        RegistrationClosed => vec![InProgress, RegistrationOpen, Cancelled], // Allow reopen
        InProgress => vec![Judging, Cancelled],
        Judging => vec![Completed, Cancelled],
        Completed => vec![], // Terminal state
        Cancelled => vec![], // Terminal state
    };
    
    if allowed_transitions.contains(next) {
        Ok(())
    } else {
        bail!(
            "Invalid status transition: {:?} -> {:?}. Allowed transitions from {:?} are: {:?}",
            current,
            next,
            current,
            allowed_transitions
        )
    }
}

/// Validate hackathon is ready for registration
pub fn validate_ready_for_registration(hackathon: &HackathonSchema) -> Result<()> {
    // Check required fields for registration
    if hackathon.theme.is_none() || hackathon.theme.as_ref().unwrap().trim().is_empty() {
        bail!("Theme is required before opening registration");
    }
    
    if hackathon.rules.is_none() || hackathon.rules.as_ref().unwrap().trim().is_empty() {
        bail!("Rules are required before opening registration");
    }
    
    if hackathon.prizes.is_none() || hackathon.prizes.as_ref().unwrap().is_empty() {
        bail!("At least one prize is required before opening registration");
    }
    
    if hackathon.organizers.is_empty() {
        bail!("At least one organizer is required");
    }
    
    // Check dates are valid
    let now = Utc::now();
    if hackathon.registration_deadline < now {
        bail!("Registration deadline must be in the future");
    }
    
    if hackathon.start_date < now {
        bail!("Start date must be in the future");
    }
    
    Ok(())
}

/// Validate timeline phases
pub fn validate_timeline_phases(
    hackathon: &HackathonSchema,
    timelines: &[HackathonTimelineSchema],
) -> Result<()> {
    if timelines.is_empty() {
        bail!("At least one timeline phase is required");
    }
    
    if timelines.len() > MAX_TIMELINES {
        bail!("Maximum {} timeline phases allowed", MAX_TIMELINES);
    }
    
    // Check required phases exist
    let phases: Vec<HackathonPhase> = timelines.iter().map(|t| t.phase.clone()).collect();
    
    if !phases.contains(&HackathonPhase::Registration) {
        bail!("Registration phase is required");
    }
    
    if !phases.contains(&HackathonPhase::Submission) {
        bail!("Submission phase is required");
    }
    
    // Check for duplicate phases
    let unique_phases: HashSet<String> = timelines.iter()
        .map(|t| t.phase.to_string())
        .collect();
    if unique_phases.len() != timelines.len() {
        bail!("Duplicate timeline phases found");
    }
    
    // Check order is sequential
    let mut orders: Vec<u32> = timelines.iter().map(|t| t.order).collect();
    orders.sort();
    for (i, &order) in orders.iter().enumerate() {
        if order != i as u32 {
            bail!("Timeline phases must have sequential order (expected {}, got {})", i, order);
        }
    }
    
    // Check for overlapping timelines
    let mut sorted_timelines = timelines.to_vec();
    sorted_timelines.sort_by(|a, b| a.start_date.cmp(&b.start_date));
    
    for i in 0..sorted_timelines.len() - 1 {
        if sorted_timelines[i].end_date > sorted_timelines[i + 1].start_date {
            bail!(
                "Timeline phases cannot overlap: '{}' (ends {}) overlaps with '{}' (starts {})",
                sorted_timelines[i].title,
                sorted_timelines[i].end_date,
                sorted_timelines[i + 1].title,
                sorted_timelines[i + 1].start_date
            );
        }
    }
    
    // Check timeline covers entire hackathon duration
    let first = sorted_timelines.first().unwrap();
    let last = sorted_timelines.last().unwrap();
    
    // Allow small tolerance (1 hour) for timezone differences
    let tolerance = chrono::Duration::hours(1);
    
    if (first.start_date - hackathon.start_date).abs() > tolerance {
        bail!(
            "Timeline must start at hackathon start date (Timeline: {}, Hackathon: {})",
            first.start_date,
            hackathon.start_date
        );
    }
    
    if (last.end_date - hackathon.end_date).abs() > tolerance {
        bail!(
            "Timeline must end at hackathon end date (Timeline: {}, Hackathon: {})",
            last.end_date,
            hackathon.end_date
        );
    }
    
    // Check only one timeline is active
    let active_count = timelines.iter().filter(|t| t.is_active).count();
    if active_count > 1 {
        bail!("Only one timeline phase can be active at a time");
    }
    
    Ok(())
}

/// Validate organizers list
pub fn validate_organizers(organizers: &[String]) -> Result<()> {
    if organizers.is_empty() {
        bail!("At least one organizer is required");
    }
    
    if organizers.len() > MAX_ORGANIZERS {
        bail!("Maximum {} organizers allowed", MAX_ORGANIZERS);
    }
    
    // Check for duplicates
    let unique: HashSet<&String> = organizers.iter().collect();
    if unique.len() != organizers.len() {
        bail!("Duplicate organizers found");
    }
    
    // Check for empty or invalid IDs
    for organizer in organizers {
        if organizer.trim().is_empty() {
            bail!("Organizer ID cannot be empty");
        }
    }
    
    Ok(())
}

/// Validate prizes
pub fn validate_prizes(prizes: &[super::hackathon_schema::Prize]) -> Result<()> {
    if prizes.is_empty() {
        bail!("At least one prize is required");
    }
    
    if prizes.len() > MAX_PRIZES {
        bail!("Maximum {} prizes allowed", MAX_PRIZES);
    }
    
    // Check for duplicate positions
    let positions: Vec<u32> = prizes.iter().map(|p| p.position).collect();
    let unique_positions: HashSet<u32> = positions.iter().copied().collect();
    if unique_positions.len() != positions.len() {
        bail!("Duplicate prize positions found");
    }
    
    // Check positions are valid (starting from 1)
    for prize in prizes {
        if prize.position == 0 {
            bail!("Prize position must start from 1");
        }
        if prize.title.trim().is_empty() {
            bail!("Prize title cannot be empty");
        }
    }
    
    Ok(())
}

/// Validate dates consistency
pub fn validate_dates(
    start_date: &chrono::DateTime<Utc>,
    end_date: &chrono::DateTime<Utc>,
    registration_deadline: &chrono::DateTime<Utc>,
) -> Result<()> {
    if end_date <= start_date {
        bail!("End date must be after start date");
    }
    
    if registration_deadline >= end_date {
        bail!("Registration deadline must be before end date");
    }
    
    // Registration deadline should ideally be before or at start date
    if registration_deadline > start_date {
        // Allow but warn - some hackathons allow registration during event
        tracing::warn!(
            "Registration deadline ({}) is after start date ({})",
            registration_deadline,
            start_date
        );
    }
    
    Ok(())
}

/// Validate hackathon can be deleted
pub fn validate_can_delete(hackathon: &HackathonSchema) -> Result<()> {
    // Cannot delete completed hackathons (for historical records)
    if hackathon.status == HackathonStatus::Completed {
        bail!("Cannot delete completed hackathons. They are kept for historical records.");
    }
    
    Ok(())
}

/// Get current active phase based on timeline
pub fn get_current_phase(timelines: &[HackathonTimelineSchema]) -> Option<HackathonPhase> {
    let now = Utc::now();
    
    // Find the timeline that contains current time
    for timeline in timelines {
        if timeline.start_date <= now && timeline.end_date >= now {
            return Some(timeline.phase.clone());
        }
    }
    
    None
}

/// Validate submission is allowed in current phase
pub fn validate_submission_allowed(timelines: &[HackathonTimelineSchema]) -> Result<()> {
    let current_phase = get_current_phase(timelines);
    
    match current_phase {
        Some(HackathonPhase::Submission) => Ok(()),
        Some(phase) => bail!("Submissions are not allowed in {:?} phase", phase),
        None => bail!("No active phase found"),
    }
}

/// Validate registration is allowed
pub fn validate_registration_allowed(
    hackathon: &HackathonSchema,
    current_participant_count: u32,
) -> Result<()> {
    // Check status
    if hackathon.status != HackathonStatus::RegistrationOpen {
        bail!("Registration is not open for this hackathon");
    }
    
    // Check deadline
    let now = Utc::now();
    if hackathon.registration_deadline < now {
        bail!("Registration deadline has passed");
    }
    
    // Check max participants
    if let Some(max) = hackathon.max_participants {
        if current_participant_count >= max {
            bail!("Maximum participant limit reached");
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_status_transitions() {
        assert!(can_transition_status(&HackathonStatus::Draft, &HackathonStatus::RegistrationOpen).is_ok());
        assert!(can_transition_status(&HackathonStatus::Draft, &HackathonStatus::Completed).is_err());
        assert!(can_transition_status(&HackathonStatus::Completed, &HackathonStatus::Draft).is_err());
    }

    #[test]
    fn test_validate_organizers() {
        assert!(validate_organizers(&vec!["org1".to_string()]).is_ok());
        assert!(validate_organizers(&vec![]).is_err());
        assert!(validate_organizers(&vec!["org1".to_string(), "org1".to_string()]).is_err());
    }

    #[test]
    fn test_validate_dates() {
        let now = Utc::now();
        let start = now + Duration::days(1);
        let end = now + Duration::days(7);
        let deadline = now + Duration::hours(12);
        
        assert!(validate_dates(&start, &end, &deadline).is_ok());
        assert!(validate_dates(&end, &start, &deadline).is_err()); // end before start
    }
}
