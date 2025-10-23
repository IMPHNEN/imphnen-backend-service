use axum::http::StatusCode;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum AppError {
    ValidationError(String),
    AuthenticationError(String),
    AuthorizationError(String),
    NotFoundError(String),
    ConflictError(String),
    InternalServerError(String),
    BadRequestError(String),
    ForbiddenError(String),
    PaymentRequiredError(String),
    MethodNotAllowedError(String),
    NotAcceptableError(String),
    RequestTimeoutError(String),
    TooManyRequestsError(String),
    GatewayTimeoutError(String),
    ServiceUnavailableError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::AuthenticationError(msg) => write!(f, "Authentication failed: {}", msg),
            AppError::AuthorizationError(msg) => write!(f, "Authorization failed: {}", msg),
            AppError::NotFoundError(msg) => write!(f, "Resource not found: {}", msg),
            AppError::ConflictError(msg) => write!(f, "Conflict error: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
            AppError::BadRequestError(msg) => write!(f, "Bad request: {}", msg),
            AppError::ForbiddenError(msg) => write!(f, "Forbidden: {}", msg),
            AppError::PaymentRequiredError(msg) => write!(f, "Payment required: {}", msg),
            AppError::MethodNotAllowedError(msg) => write!(f, "Method not allowed: {}", msg),
            AppError::NotAcceptableError(msg) => write!(f, "Not acceptable: {}", msg),
            AppError::RequestTimeoutError(msg) => write!(f, "Request timeout: {}", msg),
            AppError::TooManyRequestsError(msg) => write!(f, "Too many requests: {}", msg),
            AppError::GatewayTimeoutError(msg) => write!(f, "Gateway timeout: {}", msg),
            AppError::ServiceUnavailableError(msg) => write!(f, "Service unavailable: {}", msg),
        }
    }
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
            AppError::AuthorizationError(_) => StatusCode::FORBIDDEN,
            AppError::NotFoundError(_) => StatusCode::NOT_FOUND,
            AppError::ConflictError(_) => StatusCode::CONFLICT,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequestError(_) => StatusCode::BAD_REQUEST,
            AppError::ForbiddenError(_) => StatusCode::FORBIDDEN,
            AppError::PaymentRequiredError(_) => StatusCode::PAYMENT_REQUIRED,
            AppError::MethodNotAllowedError(_) => StatusCode::METHOD_NOT_ALLOWED,
            AppError::NotAcceptableError(_) => StatusCode::NOT_ACCEPTABLE,
            AppError::RequestTimeoutError(_) => StatusCode::REQUEST_TIMEOUT,
            AppError::TooManyRequestsError(_) => StatusCode::TOO_MANY_REQUESTS,
            AppError::GatewayTimeoutError(_) => StatusCode::GATEWAY_TIMEOUT,
            AppError::ServiceUnavailableError(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    pub fn message(&self) -> String {
        self.to_string()
    }
}

pub type Result<T, E = AppError> = std::result::Result<T, E>;