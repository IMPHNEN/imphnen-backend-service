use imphnen_libs::postgres::{PostgresConfig, PostgresConnection};
use sea_orm::{Database, DatabaseConnection, DbErr, EntityTrait};
use std::env;

#[tokio::test]
async fn test_postgresql_connection() -> Result<(), DbErr> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Create PostgreSQL config from env
    let config = PostgresConfig::from_env().map_err(|e| {
        DbErr::Custom(format!("Config error: {e}").into())
    })?;

    // Create connection
    let connection = PostgresConnection::new(config).await.map_err(|e| {
        DbErr::Custom(format!("Connection error: {e}").into())
    })?;

    // Test basic query - check if we can connect and run a simple query
    let result = connection.conn.execute_unprepared("SELECT 1").await?;

    assert_eq!(result.rows_affected(), 0); // SELECT doesn't affect rows

    println!("PostgreSQL connection successful!");
    Ok(())
}

#[tokio::test]
async fn test_seaorm_basic_query() -> Result<(), DbErr> {
    // This test verifies that SeaORM can connect to PostgreSQL
    // Note: This doesn't use the custom entities due to compilation issues

    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://root:root@localhost:5432/localdb".to_string());

    let db: DatabaseConnection = Database::connect(&database_url).await?;

    // Test basic SeaORM functionality with a simple raw query
    let result: Vec<(i32,)> = sea_orm::Statement::from_sql_and_values(
        db.get_database_backend(),
        "SELECT 1 as test_column",
        vec![],
    )
    .query_all(&db)
    .await?;

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].0, 1);

    println!("SeaORM basic query successful!");
    Ok(())
}