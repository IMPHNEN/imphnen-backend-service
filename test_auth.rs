use std::env;
use dotenvy::dotenv;
use imphnen_libs::postgres::{PostgresConfig, PostgresConnection};
use imphnen_entities::seaorm::auth::users::{Entity as UsersEntity, Column as UserColumn};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
use imphnen_libs::{hash_password, verify_password};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();
    
    // Create PostgreSQL configuration
    let config = PostgresConfig::from_env()?;
    println!("Database URL: {}", config.database_url);
    
    // Create PostgreSQL connection
    let pg_conn = PostgresConnection::new(config).await?;
    let db = &pg_conn.conn;
    
    // Test database connection
    let statement = sea_orm::Statement::from_string(
        sea_orm::DatabaseBackend::Postgres,
        "SELECT version()".into(),
    );
    let result = pg_conn.execute(statement).await?;
    println!("PostgreSQL version: {:?}", result);
    
    // Query the admin user
    let email = "admin@example.com";
    println!("\nLooking for user with email: {}", email);
    
    let user = UsersEntity::find()
        .filter(UserColumn::Email.eq(email))
        .filter(UserColumn::DeletedAt.is_null())
        .one(db)
        .await?;
    
    match user {
        Some(user) => {
            println!("User found:");
            println!("  ID: {}", user.id);
            println!("  Email: {}", user.email);
            println!("  Username: {}", user.username);
            println!("  Is Active: {}", user.is_active);
            println!("  Password Hash: {}", user.password_hash);
            
            // Test password verification
            let password = "password";
            println!("\nTesting password verification for '{}'", password);
            
            let is_valid = verify_password(password, &user.password_hash)?;
            println!("Password is valid: {}", is_valid);
            
            // Test hashing the same password to compare
            let new_hash = hash_password(password)?;
            println!("New hash of same password: {}", new_hash);
            
            // Compare hashes
            println!("Hashes match: {}", user.password_hash == new_hash);
        }
        None => {
            println!("User not found!");
        }
    }
    
    Ok(())
}