use anyhow::{ Context, Ok, Result };
use dotenv::{ dotenv, var };
use supabase_rs::SupabaseClient;

use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
struct SerialNumbersTable {
    serial_number: String,
    device_id: Option<String>,
    active: bool,
    users: User,
}
#[derive(Serialize, Deserialize, Debug)]
struct User {
    email: String,
}

async fn connect_supabase() -> Result<SupabaseClient> {
    dotenv().ok();

    let url = var("SUPABASE_URL").with_context(|| "SUPABASE_URL not set")?;
    let anon_key = var("SUPABASE_KEY").with_context(|| "SUPABASE_KEY not set")?;
    let supabase_client = SupabaseClient::new(url, anon_key)?;
    Ok(supabase_client)
}

async fn fetch_serial_number_table(
    supabase_client: SupabaseClient,
    serial_number: &str
) -> Result<SerialNumbersTable> {
    let get_active_status = supabase_client
        .select("serial_numbers")
        .columns(["*", "users(email)"].to_vec())
        .eq("serial_number", serial_number)
        .execute().await;

    if get_active_status.is_err() {
        println!("Error fetching active status: {:?}", get_active_status);
        return Err(anyhow::anyhow!("Failed to fetch active status"));
    }

    let mut serial_numbers: Option<SerialNumbersTable> = None;
    if get_active_status.is_ok() {
        for response in get_active_status.iter() {
            println!("Active status: {:?}", response);
            if response.is_empty() {
                println!("No active status found for the given serial number");
                return Err(anyhow::anyhow!("No active status found for the given serial number"));
            }

            serial_numbers = Some(
                serde_json
                    ::from_value(response[0].clone())
                    .with_context(|| "Failed to deserialize response into SerialNumbersTable")?
            );
        }
    }

    let table = match serial_numbers {
        Some(sn) => sn,
        None => {
            println!("No serial number data found");
            return Err(anyhow::anyhow!("Serial number data could not be retrieved"));
        }
    };

    Ok(table)
}

fn authorization(serial_number_table: SerialNumbersTable, device_id: &str) -> Result<()> {
    println!("EARL_DEBUG compare active and device_id");
    let current_device_id = match serial_number_table.device_id {
        Some(id) => id,
        None => { "".to_string() }
    };

    if serial_number_table.active && current_device_id == device_id {
        println!("Serial number is already active and device ID matches");
        return Ok(());
    }

    println!("EARL_DEBUG active should be false");
    if !serial_number_table.active {
        println!("Serial number is not active and device ID does not match");
        return Ok(());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use supabase_rs::update;

    use super::*;

    #[tokio::test]
    async fn test_success_when_serial_number_is_found_and_active_is_false() {
        let client = connect_supabase().await.unwrap();
        let serial_number_table = fetch_serial_number_table(client, "SERIAL-123").await;
        assert!(serial_number_table.is_ok(), "Serial number is found on the database");
        let auth_result = authorization(serial_number_table.unwrap(), "mock_device_id");
        assert!(auth_result.is_ok(), "Authorization should be successful");
    }

    #[tokio::test]
    async fn test_success_when_serialnumber_is_found_active_is_true_deviceid_is_match() {
        let client = connect_supabase().await.unwrap();
        let serial_number_table = fetch_serial_number_table(client, "SERIAL-456").await;
        assert!(serial_number_table.is_ok(), "Serial number should be found on the database");
        let auth_result = authorization(serial_number_table.unwrap(), "windowsmachine");
        assert!(auth_result.is_ok(), "Authorization should be successful");
    }

    #[tokio::test]
    async fn test_failure_when_serial_number_is_not_found() {
        let client = connect_supabase().await.unwrap();
        let result = fetch_serial_number_table(client, "INVALIDSERIAL-123").await;
        assert!(result.is_err(), "Serial number should not be found on the database");
    }
}
