use anyhow::{ Context, Ok, Result };
use dotenv::{ dotenv, var };
use serde_json::json;
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

pub async fn authorize(serial_number: &str, email_address: &str, device_id: &str) -> Result<bool> {
    let supabase_client = connect_supabase().await?;

    let serial_number_table = fetch_serial_number_table(
        supabase_client.clone(),
        serial_number
    ).await?;
    let is_authorized = authorization(serial_number_table, device_id, email_address);

    // TODO: update the table
    // if is_authorized {
    //     update_status(supabase_client, serial_number, device_id, true).await?;
    // }

    Ok(is_authorized)
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

async fn update_status(
    supabase_client: SupabaseClient,
    serial_number: &str,
    device_id: &str,
    active: bool
) -> Result<()> {
    let update_result = supabase_client.update_with_column_name(
        "serial_numbers",
        "serial_number",
        serial_number,
        json!({
           "active": active,
           "device_id": device_id
         })
    ).await;

    if update_result.is_err() {
        println!("Error updating status: {:?}", update_result);
        return Err(anyhow::anyhow!("Failed to update status"));
    }

    Ok(())
}

fn authorization(
    serial_number_table: SerialNumbersTable,
    device_id: &str,
    email_address: &str
) -> bool {
    println!("EARL_DEBUG compare active and device_id");
    let current_device_id = match serial_number_table.device_id {
        Some(id) => id,
        None => { "".to_string() }
    };

    if email_address != serial_number_table.users.email {
        println!("Transaction failed. Email address is not the owner of the serial number");
        return false;
    }

    if serial_number_table.active && current_device_id != device_id {
        println!("Serial number is already active and device ID does not match");
        return false;
    }

    if serial_number_table.active && current_device_id == device_id {
        println!("Serial number is already active and device ID matches");
        return true;
    }

    println!("EARL_DEBUG active should be false");
    if !serial_number_table.active {
        println!("Serial number is not active and device ID does not match");

        // TODO: Update the serial number
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_EMAIL_ADDRESS: &str = "mpguser004@gmail.com";

    #[tokio::test]
    async fn test_success_when_serial_number_is_found_and_active_is_false() {
        let client = connect_supabase().await.unwrap();
        let serial_number_table = fetch_serial_number_table(client, "SERIAL-123").await;
        assert!(serial_number_table.is_ok(), "Serial number is found on the database");
        let auth_result = authorization(
            serial_number_table.unwrap(),
            "mock_device_id",
            TEST_EMAIL_ADDRESS
        );
        assert!(auth_result, "Authorization should be successful");
    }

    #[tokio::test]
    async fn test_success_when_serialnumber_is_found_active_is_true_deviceid_is_match() {
        let client = connect_supabase().await.unwrap();
        let serial_number_table = fetch_serial_number_table(client, "SERIAL-456").await;
        assert!(serial_number_table.is_ok(), "Serial number should be found on the database");
        let auth_result = authorization(
            serial_number_table.unwrap(),
            "windowsmachine",
            TEST_EMAIL_ADDRESS
        );
        assert!(auth_result, "Authorization should be successful");
    }

    #[tokio::test]
    async fn test_authorize_failure_when_active_is_true_and_deviceid_is_incorrect() {
        let client = connect_supabase().await.unwrap();
        let serial_number_table = fetch_serial_number_table(client, "SERIAL-456").await;
        assert!(serial_number_table.is_ok(), "Serial number should be found on the database");
        let auth_result = authorization(
            serial_number_table.unwrap(),
            "incorrect_device_id",
            TEST_EMAIL_ADDRESS
        );
        assert!(!auth_result, "Authorization should NOT be successful");
    }

    #[tokio::test]
    async fn test_failure_when_serial_number_is_not_found() {
        let client = connect_supabase().await.unwrap();
        let result = fetch_serial_number_table(client, "INVALIDSERIAL-123").await;
        assert!(result.is_err(), "Serial number should not be found on the database");
    }

    #[tokio::test]
    async fn test_failure_when_email_address_is_not_owner_of_serial() {
        let client = connect_supabase().await.unwrap();
        let serial_number_table = fetch_serial_number_table(client, "SERIAL-123").await;
        assert!(serial_number_table.is_ok(), "Serial number should be found on the database");
        let auth_result = authorization(
            serial_number_table.unwrap(),
            "mockdevice_id",
            "invalid@emailaddress.com"
        );
        assert!(!auth_result, "Authorization should NOT be successful");
    }

    #[tokio::test]
    async fn test_success_update_status() {
        let client = connect_supabase().await.unwrap();
        let result = update_status(client.clone(), "SERIAL-789", "earl_device_id", true).await;
        assert!(result.is_ok(), "Status should be updated successfully");

        // Revert for testing purposes
        let _result = update_status(client, "SERIAL-789", "earl_device_id", false).await;
    }
}
