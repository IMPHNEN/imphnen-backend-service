//! SurrealDB client initialization and configuration.
//!
//! This module provides utilities for initializing SurrealDB connections
//! for both WebSocket and in-memory databases, along with resource definitions.

use crate::environment::ENV;
use surrealdb::engine::any;
use surrealdb::engine::local::{Db, Mem};
use surrealdb::opt::auth::Root;
use surrealdb::{Result, Surreal};

/// Type alias for SurrealDB WebSocket client.
pub type SurrealWsClient = Surreal<any::Any>;

/// Type alias for SurrealDB in-memory client.
pub type SurrealMemClient = Surreal<Db>;

pub mod resource;
pub use resource::*;

/// Initialize a SurrealDB WebSocket client connection.
///
/// This function creates a connection to a SurrealDB instance via WebSocket,
/// authenticates with root credentials, and sets the namespace and database.
///
/// # Returns
/// * `Ok(SurrealWsClient)` - Successfully initialized WebSocket client
/// * `Err(surrealdb::Error)` - Connection, authentication, or configuration failed
///
/// # Example
/// ```no_run
/// use imphnen_libs::surrealdb_init_ws;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = surrealdb_init_ws().await?;
///     // Use client for database operations
///     Ok(())
/// }
/// ```
pub async fn surrealdb_init_ws() -> Result<Surreal<any::Any>> {
    let env = &ENV;

    log::info!("Initializing SurrealDB WebSocket connection to: {}", env.surrealdb_url);

    // Connect to SurrealDB
    let db = any::connect(&env.surrealdb_url).await?;
    log::debug!("WebSocket connection established");

    // Authenticate
    db.signin(Root {
        username: &env.surrealdb_username,
        password: &env.surrealdb_password,
    })
    .await?;
    log::debug!("Authentication successful");

    // Configure namespace and database
    db.use_ns(&env.surrealdb_namespace)
        .use_db(&env.surrealdb_dbname)
        .await?;
    log::info!("SurrealDB WebSocket client initialized with namespace '{}' and database '{}'",
               env.surrealdb_namespace, env.surrealdb_dbname);

    Ok(db)
}

/// Initialize a SurrealDB in-memory client.
///
/// This function creates an in-memory SurrealDB instance and configures
/// the namespace and database for use.
///
/// # Returns
/// * `Ok(SurrealMemClient)` - Successfully initialized in-memory client
/// * `Err(surrealdb::Error)` - Initialization or configuration failed
///
/// # Example
/// ```no_run
/// use imphnen_libs::surrealdb_init_mem;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = surrealdb_init_mem().await?;
///     // Use client for in-memory database operations
///     Ok(())
/// }
/// ```
pub async fn surrealdb_init_mem() -> Result<SurrealMemClient> {
    let env = &ENV;

    log::info!("Initializing SurrealDB in-memory database");

    // Create in-memory database
    let db = Surreal::new::<Mem>(()).await?;
    log::debug!("In-memory database created");

    // Configure namespace and database
    db.use_ns(&env.surrealdb_namespace)
        .use_db(&env.surrealdb_dbname)
        .await?;
    log::info!("SurrealDB in-memory client initialized with namespace '{}' and database '{}'",
               env.surrealdb_namespace, env.surrealdb_dbname);

    Ok(db)
}
