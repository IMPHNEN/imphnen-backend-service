/// Version 2 of the Dimentorin API - currently under development
/// This module will contain all version 2 endpoints following API versioning best practices

use axum::Router;

/// Placeholder for version 2 router
/// To be implemented when version 2 endpoints are ready
pub fn dimentorin_v2_router() -> Router {
	Router::new()
		// Version 2 endpoints will be added here following the same pattern as v1
}

// Re-export the v1 router for backward compatibility
pub use crate::v1::dimentorin_router;
