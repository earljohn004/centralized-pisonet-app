use anyhow::{ Context, Result };
use dotenv::{ dotenv, var };
use serde_json::json;
use supabase_rs::SupabaseClient;

use reqwest;
use serde::{ Deserialize, Serialize };
use std::{ error::Error, f32::consts::E, ptr::null };

#[derive(Serialize, Deserialize, Debug)]
struct SerialNumbersTable {
    serial_number: String,
    device_id: Option<String>,
    active: bool,
}

async fn connect_supabase() -> Result<SupabaseClient> {
    dotenv().ok();

    let url = var("SUPABASE_URL").with_context(|| "SUPABASE_URL not set")?;
    let anon_key = var("SUPABASE_KEY").with_context(|| "SUPABASE_KEY not set")?;
    let supabase_client = SupabaseClient::new(url, anon_key)?;
    Ok(supabase_client)
}

async fn update_serial_number(
    supabase_client: SupabaseClient,
    device_id: &str,
    serial_number: &str
) -> Result<()> {
    let get_active_status = supabase_client
        .select("serial_numbers")
        .columns(["active", "serial_number", "device_id"].to_vec())
        .eq("serial_number", serial_number)
        .execute().await;

    if get_active_status.is_err() {
        return Err(anyhow::anyhow!("Failed to fetch active status"));
    }

    let mut serial_numbers: Option<SerialNumbersTable> = None;
    if get_active_status.is_ok() {
        for response in get_active_status.iter() {
            println!("Active status: {:?}", response);
            if response.is_empty() {
                return Err(anyhow::anyhow!("No active status found for the given serial number"));
            }

            println!("EARL_DEBUG assign to struct");
            serial_numbers = Some(
                serde_json
                    ::from_value(response[0].clone())
                    .with_context(|| "Failed to deserialize response into SerialNumbersTable")?
            );
        }
    }

    println!("EARL_DEBUG exit");
    let serial_numbers = match serial_numbers {
        Some(sn) => sn,
        None => {
            return Err(anyhow::anyhow!("Serial number data could not be retrieved"));
        }
    };

    println!("EARL_DEBUG compare active and device_id");
    let current_device_id = match serial_numbers.device_id {
        Some(id) => id,
        None => { "".to_string() }
    };

    if serial_numbers.active && current_device_id == device_id {
        println!("Serial number is already active and device ID matches");
        return Ok(());
    }

    println!("EARL_DEBUG active should be false");
    if !serial_numbers.active {
        println!("Serial number is not active and device ID does not match");
        return Ok(());
    }

    Ok(())
}

async fn find_user_in_db(client: SupabaseClient, email: &str) -> Result<()> {
    let user_query = client
        .select("users")
        .columns(["email"].to_vec())
        .eq("email", email)
        .execute().await;

    if user_query.is_err() {
        println!("Error fetching user: {:?}", user_query);
        return Err(anyhow::anyhow!("Failed to fetch user"));
    }

    if user_query.is_ok() {
        for users in user_query.iter() {
            if users.is_empty() {
                println!("No user found with the given email");
                return Err(anyhow::anyhow!("No user found with the given email"));
            }

            println!("User found: {:?}", users);
            if users.iter().count() == 0 {
                println!("No user found with the given email");
                return Err(anyhow::anyhow!("No user found with the given email"));
            }

            if users.iter().any(|user| user == email) {
                println!("User found: {:?}", users);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use supabase_rs::update;

    use super::*;

    #[tokio::test]
    async fn test_success_when_user_is_found() {
        let client = connect_supabase().await.unwrap();
        let result = find_user_in_db(client, "mpguser004@gmail.com").await;
        assert!(result.is_ok(), "User is found on the database");
    }

    #[tokio::test]
    async fn test_failure_when_user_is_not_found() {
        let client = connect_supabase().await.unwrap();
        let result = find_user_in_db(client, "testemail@gmail.com").await;
        assert!(result.is_err(), "User should not be found on the database");
    }

    #[tokio::test]
    async fn test_success_when_serial_number_is_found_and_active_is_false() {
        let client = connect_supabase().await.unwrap();
        let result = update_serial_number(client, "mock_device_id", "SERIAL-123").await;
        assert!(result.is_ok(), "Serial number is found on the database");
    }

    #[tokio::test]
    async fn test_success_when_serialnumber_is_found_active_is_true_deviceid_is_match() {
        let client = connect_supabase().await.unwrap();
        let result = update_serial_number(client, "windowsmachine", "SERIAL-456").await;
        assert!(result.is_ok(), "Serial number should be found on the database");
    }

    #[tokio::test]
    async fn test_failure_when_serial_number_is_not_found() {
        let client = connect_supabase().await.unwrap();
        let result = update_serial_number(client, "mock_device_id", "INVALIDSERIAL-123").await;
        assert!(result.is_err(), "Serial number should not be found on the database");
    }
}
