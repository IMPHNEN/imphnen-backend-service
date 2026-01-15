//! Custom error types for migration validation operations

use std::fmt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use imphnen_entities::seaorm::common::enums::ResourceEnum;

/// Detailed validation error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationErrorDetails {
    pub record_id: Uuid,
    pub field: String,
    pub expected_value: Option<serde_json::Value>,
    pub actual_value: Option<serde_json::Value>,
    pub error_message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ValidationErrorDetails {
    /// Create a new validation error details instance
    pub fn new(
        record_id: Uuid,
        field: String,
        error_message: String,
    ) -> Self {
        Self {
            record_id,
            field,
            expected_value: None,
            actual_value: None,
            error_message,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Create a new validation error details instance with value comparison
    pub fn with_values(
        record_id: Uuid,
        field: String,
        expected_value: serde_json::Value,
        actual_value: serde_json::Value,
        error_message: String,
    ) -> Self {
        Self {
            record_id,
            field,
            expected_value: Some(expected_value),
            actual_value: Some(actual_value),
            error_message,
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Migration validation error type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MigrationValidationError {
    /// Resource not found in one of the databases
    RecordNotFound {
        resource_type: ResourceEnum,
        record_id: Uuid,
        database: String,
    },

    /// Data mismatch between databases
    DataMismatch {
        resource_type: ResourceEnum,
        record_id: Uuid,
        errors: Vec<ValidationErrorDetails>,
    },

    /// Schema mismatch between databases
    SchemaMismatch {
        resource_type: ResourceEnum,
        missing_fields: Vec<String>,
        extra_fields: Vec<String>,
    },

    /// Validation failed for a specific record
    ValidationFailed {
        resource_type: ResourceEnum,
        record_id: Uuid,
        error: String,
    },

    /// Database connection error
    DatabaseConnectionError {
        database: String,
        error: String,
    },

    /// Query execution error
    QueryExecutionError {
        resource_type: ResourceEnum,
        database: String,
        error: String,
    },

    /// Conversion error between database models
    ModelConversionError {
        resource_type: ResourceEnum,
        error: String,
    },

    /// Validation timeout
    ValidationTimeout {
        resource_type: ResourceEnum,
        duration: String,
    },

    /// Partial validation completed (some records failed)
    PartialValidation {
        resource_type: ResourceEnum,
        total_records: i32,
        failed_records: i32,
        errors: Vec<ValidationErrorDetails>,
    },

    /// Unsupported validation operation
    UnsupportedOperation {
        resource_type: ResourceEnum,
        operation: String,
    },

    /// Validation skipped for resource
    ValidationSkipped {
        resource_type: ResourceEnum,
        reason: String,
    },
}

impl fmt::Display for MigrationValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MigrationValidationError::RecordNotFound { resource_type, record_id, database } => {
                write!(f, "Record not found in {} for resource {}: {}", database, resource_type.as_str(), record_id)
            }
            MigrationValidationError::DataMismatch { resource_type, record_id, errors } => {
                write!(f, "Data mismatch in resource {} for record {}: {} errors", resource_type.as_str(), record_id, errors.len())
            }
            MigrationValidationError::SchemaMismatch { resource_type, missing_fields, extra_fields } => {
                write!(f, "Schema mismatch in resource {}: {} missing fields, {} extra fields", resource_type.as_str(), missing_fields.len(), extra_fields.len())
            }
            MigrationValidationError::ValidationFailed { resource_type, record_id, error } => {
                write!(f, "Validation failed for resource {} record {}: {}", resource_type.as_str(), record_id, error)
            }
            MigrationValidationError::DatabaseConnectionError { database, error } => {
                write!(f, "{} connection error: {}", database, error)
            }
            MigrationValidationError::QueryExecutionError { resource_type, database, error } => {
                write!(f, "Query execution error in {} for resource {}: {}", database, resource_type.as_str(), error)
            }
            MigrationValidationError::ModelConversionError { resource_type, error } => {
                write!(f, "Model conversion error for resource {}: {}", resource_type.as_str(), error)
            }
            MigrationValidationError::ValidationTimeout { resource_type, duration } => {
                write!(f, "Validation timeout for resource {} after {}: {}", resource_type.as_str(), duration, duration)
            }
            MigrationValidationError::PartialValidation { resource_type, total_records, failed_records, errors: _ } => {
                write!(f, "Partial validation for resource {}: {}/{} records failed ({:.1}%)", resource_type.as_str(), failed_records, total_records, (*failed_records as f64 / *total_records as f64) * 100.0)
            }
            MigrationValidationError::UnsupportedOperation { resource_type, operation } => {
                write!(f, "Unsupported operation {} for resource {}", operation, resource_type.as_str())
            }
            MigrationValidationError::ValidationSkipped { resource_type, reason } => {
                write!(f, "Validation skipped for resource {}: {}", resource_type.as_str(), reason)
            }
        }
    }
}

impl std::error::Error for MigrationValidationError {}

/// Result type for migration validation operations
pub type MigrationValidationResult<T> = Result<T, MigrationValidationError>;

/// Validation summary for a resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSummary {
    pub resource_type: ResourceEnum,
    pub status: String,
    pub total_records: i32,
    pub validated_records: i32,
    pub failed_records: i32,
    pub skipped_records: i32,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub duration: String,
    pub error_count: i32,
    pub warning_count: i32,
    pub details_url: Option<String>,
}

impl ValidationSummary {
    /// Create a new validation summary
    pub fn new(resource_type: ResourceEnum, status: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            resource_type,
            status,
            total_records: 0,
            validated_records: 0,
            failed_records: 0,
            skipped_records: 0,
            start_time: now,
            end_time: now,
            duration: "0s".to_string(),
            error_count: 0,
            warning_count: 0,
            details_url: None,
        }
    }

    /// Calculate and set duration from start to end time
    pub fn calculate_duration(&mut self) {
        let duration = self.end_time.signed_duration_since(self.start_time);
        self.duration = format!("{}s", duration.num_seconds());
    }
}