use anyhow::{ Context, Result };
use dotenv::dotenv;
use serde_json::json;
use supabase_rs::SupabaseClient;
use std::env::var;

async fn connect_supabase() -> Result<SupabaseClient> {
    dotenv().ok();

    let url = var("SUPABASE_URL").with_context(|| "SUPABASE_URL not set")?;
    let anon_key = var("SUPABASE_KEY").with_context(|| "SUPABASE_KEY not set")?;
    let supabase_client = SupabaseClient::new(url, anon_key)?;
    Ok(supabase_client)
}

// async fn update_serial_number(supabase_client: &SupabaseClient) -> Result<()> {
//     let update_result = supabase_client.update_with_column_name(
//         "serial_numbers",
//         "active",
//         "false",
//         json!({
//            "active": "true"
//          })
//     ).await;

//     match update_result {
//         Ok(_) => println!("Update successful"),
//         Err(e) => println!("Error updating record: {:?}", e),
//     }

//     Ok(())
// }

/// Updates the active status of a serial number record when the email matches and device_id is empty
/// # Arguments
/// * `email` - The email address to match against auth.users
/// * `serial_number` - The serial number to update
async fn activate_serial_number(
    client: SupabaseClient,
    email: &str,
    serial_number: &str
) -> Result<()> {
    let user_query = client
        .select("users")
        .columns(["email"].to_vec())
        .eq("email", email)
        .execute().await;

    match user_query {
        Ok(value) => {
            println!("EARL_DEBUG test");
            println!("{:?}", value);
            for f in value.iter() {
                println!("User found: {:?}", f);
                if f == email {
                    println!("User found: {:?}", f);
                }
            }
        }
        Err(e) => {
            println!("Error fetching user: {:?}", e);
            return Err(anyhow::anyhow!("Failed to fetch user"));
        }
    }

    Ok(())

    // // Now update the serial_number table where uid matches and device_id is null
    // let update_result = supabase
    //     .from("serial_number")
    //     .update(json!({ "active": true }))
    //     .eq("uid", uid)
    //     .eq("serial_number", serial_number)
    //     .is_("device_id", "null") // Check if device_id is NULL
    //     .execute().await?;

    // if update_result.status().is_success() {
    //     let response_data: serde_json::Value = update_result.json().await?;
    //     if let Some(updated_count) = response_data.as_array().map(|arr| arr.len()) {
    //         if updated_count > 0 {
    //             println!("Successfully activated serial number {}", serial_number);
    //             return Ok(());
    //         }
    //     }
    //     Err(anyhow::anyhow!("No matching serial number found with the given conditions"))
    // } else {
    //     Err(anyhow::anyhow!("Failed to update serial number: {}", update_result.status()))
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connect() {
        let client = connect_supabase().await.unwrap();
        let _ = activate_serial_number(client, "mpguser004@gmail.com", "1234567").await.unwrap();
    }
}
