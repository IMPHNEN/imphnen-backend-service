use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;
use futures_util::{StreamExt, SinkExt};
use serde_json::json;

// Menentukan kredensial dan detail koneksi secara langsung sebagai string statis
static SURREALDB_URL_WS: &str = "ws://localhost:8000/rpc";
static SURREALDB_USERNAME: &str = "root";
static SURREALDB_PASSWORD: &str = "root";
static SURREALDB_NAMESPACE: &str = "test";
static SURREALDB_DBNAME: &str = "test";

// Daftar tabel sebagai variabel static yang tidak dapat diubah
static TABLES_TO_CLEAR: &[&str] = &[
    "app_events", "users", "roles", "permissions", "gacha_rolls",
    "mentor_users", "gacha_claims", "gacha_credits", "gacha_items",
    "mentor_profiles", "roles_permissions", "testimonials",
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tidak perlu memuat env lagi, karena kita menggunakan nilai hardcoded
    // imphnen_libs::enviroment::load_env(); // Baris ini tidak lagi dibutuhkan
    // let env = Env::new(); // Baris ini tidak lagi dibutuhkan

    println!("DEBUG: URL WS: {}", SURREALDB_URL_WS);
    println!("DEBUG: Username: {}", SURREALDB_USERNAME);
    println!("DEBUG: Namespace: {}", SURREALDB_NAMESPACE);
    println!("DEBUG: Database: {}", SURREALDB_DBNAME);

    let url = Url::parse(SURREALDB_URL_WS)?; // Menggunakan SURREALDB_URL_WS statis

    let (ws_stream, _) = connect_async(url).await?;
    let (mut write, mut read) = ws_stream.split();

    // Authenticate (signin)
    let signin_query = json!({
        "method": "signin",
        "params": [{
            "user": SURREALDB_USERNAME, // Menggunakan SURREALDB_USERNAME statis
            "pass": SURREALDB_PASSWORD, // Menggunakan SURREALDB_PASSWORD statis
        }],
        "id": 1
    }).to_string();
    println!("DEBUG: Sending signin query: {}", signin_query);
    write.send(Message::Text(signin_query)).await?;
    
    let signin_response = read.next().await.ok_or("Failed to read signin response")?;
    let signin_response_msg = signin_response?;
    let signin_response_str = signin_response_msg.to_text()?;
    println!("DEBUG: Signin response: {}", signin_response_str);
    if signin_response_str.contains("\"error\":") {
        return Err(format!("Signin failed: {}", signin_response_str).into());
    }

    // Use namespace and database
    let use_query = json!({
        "method": "use",
        "params": [SURREALDB_NAMESPACE, SURREALDB_DBNAME], // Menggunakan NS & DB statis
        "id": 2
    }).to_string();
    println!("DEBUG: Sending use query: {}", use_query);
    write.send(Message::Text(use_query)).await?;
    
    let use_response = read.next().await.ok_or("Failed to read use response")?;
    let use_response_msg = use_response?;
    let use_response_str = use_response_msg.to_text()?;
    println!("DEBUG: Use response: {}", use_response_str);
    if use_response_str.contains("\"error\":") {
        return Err(format!("USE command failed: {}", use_response_str).into());
    }

    println!("INFO: Attempting to clear database tables via WebSocket...");

    let mut all_clear = true;
    for (i, table) in TABLES_TO_CLEAR.iter().enumerate() {
        let remove_query = format!("REMOVE TABLE {};", table);
        let query_json = json!({
            "method": "query",
            "params": [remove_query],
            "id": i + 3
        }).to_string();

        println!("DEBUG: Attempting REMOVE TABLE {}: {}", table, query_json);
        write.send(Message::Text(query_json)).await?;
        let response_result = read.next().await.ok_or("Stream ended unexpectedly")?;

        

        match response_result {
            Ok(msg) => {
                let response_str = msg.to_text()?;
                if response_str.contains("\"error\":") {
                    println!("WARN: Failed to REMOVE TABLE {}: {}. Attempting DELETE type::{}.", table, response_str, table);
                    let delete_all_query = format!("DELETE FROM {};", table);
                    let delete_all_json = json!({
                        "method": "query",
                        "params": [delete_all_query],
                        "id": i + 300
                    }).to_string();

                    println!("DEBUG: Attempting DELETE {}: {}", table, delete_all_json);
                    write.send(Message::Text(delete_all_json)).await?;
                    let delete_response_result = read.next().await.ok_or("Stream ended unexpectedly during DELETE type::")?;

                    match delete_response_result {
                        Ok(delete_msg) => {
                            let delete_response_str = delete_msg.to_text()?;
                            if delete_response_str.contains("\"error\":") {
                                println!("ERROR: Failed to DELETE type::{} : {}", table, delete_response_str);
                                all_clear = false;
                            } else {
                                println!("INFO: Successfully DELETED type:: table: {}", table);
                                
                            }
                        },
                        Err(delete_e) => {
                            println!("ERROR: Error receiving response for DELETE type:: table {}: {}", table, delete_e);
                            all_clear = false;
                        }
                    }
                } else {
                    println!("INFO: Successfully REMOVED TABLE: {}", table);
                    
                }
            },
            Err(e) => {
                println!("ERROR: Error receiving response for REMOVE TABLE {}: {}", table, e);
                all_clear = false;
            }
        }

        // Check if table is empty after deletion attempt
        let select_query = format!("SELECT * FROM {} LIMIT 1;", table);
        let select_json = json!({
            "method": "query",
            "params": [select_query],
            "id": i + 1000
        }).to_string();
        write.send(Message::Text(select_json)).await?;
        let select_response_result = read.next().await.ok_or("Stream ended unexpectedly during SELECT check")?;
        match select_response_result {
            Ok(select_msg) => {
                let select_response_str = select_msg.to_text()?;
                if select_response_str.contains("does not exist") {
                    println!("CHECK: Table '{}' does not exist after clear attempt (success).", table);
                } else if select_response_str.contains("\"result\":[]") || select_response_str.contains("\"result\":[[]]") {
                    println!("CHECK: Table '{}' is empty after clear attempt.", table);
                } else {
                    println!("WARNING: Table '{}' is NOT empty after clear attempt! Response: {}", table, select_response_str);
                    all_clear = false;
                }
            },
            Err(e) => {
                println!("ERROR: Error receiving response for SELECT check on table {}: {}", table, e);
                all_clear = false;
            }
        }
    }

    println!("INFO: Database clearing complete.");

    if !all_clear {
        eprintln!("ERROR: One or more tables could not be cleared. Check logs for details.");
        return Err("Database clearing failed for one or more tables.".into());
    }

    Ok(())
}