use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
};
use chrono::{DateTime, Utc};
use futures::future::BoxFuture;
use imphnen_libs::AppState;
use imphnen_utils::common_response;
use std::task::{Context, Poll};
use tower::{Layer, Service};

/// Middleware to enforce timeline-based access control for hackathon operations
#[derive(Clone)]
pub struct TimelineEnforcementLayer {
    app_state: AppState,
    allowed_phases: Vec<String>,
    operation_type: TimelineOperationType,
}

#[derive(Clone, Debug)]
pub enum TimelineOperationType {
    Registration,
    Submission,
    Custom(String),
}

impl TimelineEnforcementLayer {
    /// Create a new timeline enforcement middleware layer
    pub fn new(
        app_state: AppState,
        allowed_phases: Vec<String>,
        operation_type: TimelineOperationType,
    ) -> Self {
        Self {
            app_state,
            allowed_phases,
            operation_type,
        }
    }

    /// Create middleware for registration operations
    pub fn for_registration(app_state: AppState) -> Self {
        Self::new(
            app_state,
            vec!["registration".to_string()],
            TimelineOperationType::Registration,
        )
    }

    /// Create middleware for submission operations
    pub fn for_submission(app_state: AppState) -> Self {
        Self::new(
            app_state,
            vec!["submission".to_string()],
            TimelineOperationType::Submission,
        )
    }

    /// Create middleware for custom operations with specific allowed phases
    pub fn for_custom(
        app_state: AppState,
        allowed_phases: Vec<String>,
        operation_name: String,
    ) -> Self {
        Self::new(
            app_state,
            allowed_phases,
            TimelineOperationType::Custom(operation_name),
        )
    }
}

impl<S> Layer<S> for TimelineEnforcementLayer {
    type Service = TimelineEnforcementMiddleware<S>;
    fn layer(&self, inner: S) -> Self::Service {
        TimelineEnforcementMiddleware {
            inner,
            app_state: self.app_state.clone(),
            allowed_phases: self.allowed_phases.clone(),
            operation_type: self.operation_type.clone(),
        }
    }
}

#[derive(Clone)]
pub struct TimelineEnforcementMiddleware<S> {
    inner: S,
    app_state: AppState,
    allowed_phases: Vec<String>,
    operation_type: TimelineOperationType,
}

impl<S> Service<Request<Body>> for TimelineEnforcementMiddleware<S>
where
    S: Service<Request<Body>, Response = Response<Body>, Error = Response<Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
            let mut inner = self.inner.clone();
            let app_state = self.app_state.clone();
            let allowed_phases = self.allowed_phases.clone();
            let operation_type = self.operation_type.clone();
    
            Box::pin(async move {
                // Extract hackathon ID from request path - this assumes standard routing patterns
                let hackathon_id = extract_hackathon_id_from_request(&req)?;
                
                // Get current time
                let current_time = Utc::now();
                
                // Get hackathon timeline phases
                let timeline_phases = match get_active_timeline_phases(hackathon_id, current_time, &app_state).await {
                    Ok(phases) => phases,
                    Err(e) => return Err(common_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        &format!("Failed to check timeline: {}", e),
                    )),
                };
    
                // Check if any allowed phase is currently active
                let is_allowed = timeline_phases.iter().any(|phase| {
                    allowed_phases.iter().any(|allowed| {
                        phase.phase.to_lowercase() == *allowed
                    })
                });
    
                if !is_allowed {
                    let operation_name = match &operation_type {
                        TimelineOperationType::Registration => "registration",
                        TimelineOperationType::Submission => "submission",
                        TimelineOperationType::Custom(name) => name,
                    };
    
                    let error_msg = format!(
                        "{} is not allowed outside of specified timeline phases. Current active phases: {:?}",
                        operation_name,
                        timeline_phases.iter().map(|p| p.phase.to_string()).collect::<Vec<_>>()
                    );
    
                    return Err(common_response(
                        StatusCode::FORBIDDEN,
                        &error_msg,
                    ));
                }
    
                // Validate request body for timeline operations
                let (parts, body) = req.into_parts();
                let body_json = match validate_timeline_request_body(body).await {
                    Ok(json) => json,
                    Err(e) => return Err(e),
                };
                
                // Reconstruct request with validated body
                let req = Request::from_parts(parts, axum::body::Body::from(serde_json::to_vec(&body_json).unwrap()));
    
                inner.call(req).await
            })
        }
}

/// Extract hackathon ID from request path
fn extract_hackathon_id_from_request(req: &Request<Body>) -> Result<String, Response<Body>> {
    let uri = req.uri();
    let path = uri.path();

    // Look for patterns like /hackathons/{id}/... or /hackathons/{id}
    let segments: Vec<&str> = path.split('/').filter(|&s| !s.is_empty()).collect();
    
    for (i, segment) in segments.iter().enumerate() {
        if *segment == "hackathons" && i + 1 < segments.len() {
            return Ok(segments[i + 1].to_string());
        }
    }

    Err(common_response(
        StatusCode::BAD_REQUEST,
        "Could not extract hackathon ID from request path",
    ))
}

/// Validate request body for timeline operations
pub async fn validate_timeline_request_body(
    body: Body,
) -> Result<serde_json::Value, Response<Body>> {
    let bytes = axum::body::to_bytes(body, 1024 * 1024).await // Example limit: 1MB
        .map_err(|e| common_response(
            StatusCode::BAD_REQUEST,
            &format!("Failed to read request body: {}", e),
        ))?;
    
    let body_json = serde_json::from_slice(&bytes)
        .map_err(|e| common_response(
            StatusCode::BAD_REQUEST,
            &format!("Invalid JSON in request body: {}", e),
        ))?;
    
    Ok(body_json)
}

/// Get active timeline phases for a hackathon at current time
async fn get_active_timeline_phases(
    hackathon_id: String,
    current_time: DateTime<Utc>,
    _app_state: &AppState,
) -> Result<Vec<HackathonTimelinePhase>, String> {
    // In a real implementation, this would call the hackathon service to get timeline phases
    // For now, we'll return a mock implementation that demonstrates the pattern
    
    // This is a placeholder - in production, you would call:
    // let timeline_dtos = app_state.hackathon_service.get_active_timeline_phases(hackathon_id, current_time).await?;
    
    // For demonstration purposes, we'll return a mock response
    Ok(vec![HackathonTimelinePhase {
        id: "timeline-1".to_string(),
        hackathon_id: hackathon_id.clone(),
        phase: "registration".to_string(),
        title: "Registration Phase".to_string(),
        start_date: current_time - chrono::Duration::days(1),
        end_date: current_time + chrono::Duration::days(2),
        is_active: true,
    }])
}

/// DTO for timeline phase (matches what would be returned from service)
#[derive(Debug, Clone)]
pub struct HackathonTimelinePhase {
    pub id: String,
    pub hackathon_id: String,
    pub phase: String,
    pub title: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub is_active: bool,
}