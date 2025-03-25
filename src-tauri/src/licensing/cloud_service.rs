use anyhow::Result;
use dotenv::dotenv;
use serde_json::json;
use supabase_rs::SupabaseClient;
use std::env::var;

async fn connect_supabase() -> Result<SupabaseClient> {
    dotenv().ok();

    let supabase_client: SupabaseClient = SupabaseClient::new(
        var("SUPABASE_URL").unwrap(),
        var("SUPABASE_KEY").unwrap()
    ).unwrap();

    let update_result = supabase_client.update_with_column_name(
        "serial_numbers", // the table name
        "active", // the column name to filter by
        "false", // the value to filter by (can be any value to use as key)
        json!({
           "active": "true"  // the new value
         })
    ).await;

    match update_result {
        Ok(_) => println!("Update successful"),
        Err(e) => println!("Error updating record: {:?}", e),
    }

    Ok(supabase_client)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connect() {
        let client = connect_supabase().await.unwrap();
    }
}
