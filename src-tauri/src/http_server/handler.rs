use axum::{ Json, Router };
use tauri::{ AppHandle, Emitter };
use tokio::sync::mpsc;
use anyhow::{ Context, Result };

use super::models::{ RegisterRequest, RegisterResponse, AddTimeRequest, AddTimeResponse };

pub async fn start_server(
    app_handle: AppHandle,
    tx: mpsc::Sender<(u64, AppHandle)>,
    ip_address: String,
    port: u16
) -> Result<()> {
    let app_handle_register = app_handle.clone();
    let app_handle_add_time = app_handle.clone();

    let tx_add_time = tx.clone();

    let app = Router::new()
        .route(
            "/api/v1/register",
            axum::routing::post(move |payload| { register_handler(payload, app_handle_register) })
        )
        .route(
            "/api/v1/addtime",
            axum::routing::post(move |payload| {
                add_time_handler(payload, app_handle_add_time, tx_add_time)
            })
        );

    let address = format!("{}:{}", ip_address, port);
    let listener = tokio::net::TcpListener
        ::bind(address).await
        .with_context(|| "Failed to bind to address and port")?;

    axum::serve(listener, app).await.with_context(|| "Failed to start server")?;

    Ok(())
}

async fn register_handler(
    Json(payload): Json<RegisterRequest>,
    app_handle: AppHandle
) -> Json<RegisterResponse> {
    let pair_id = payload.pair_id;
    let _address = payload.address;
    let _hwid = payload.hwid;

    if pair_id != "pair-id-123" {
        return Json(RegisterResponse {
            status: false,
            server_hwid: "".to_string(),
            server_address: "".to_string(),
            text: "Invalid pair_id".to_string(),
        });
    }

    let status = true;
    let server_hwid = "server-hwid-123".to_string();
    let server_address = "127.0.0.1:3000".to_string();
    let text = "Registration successful".to_string();

    let register_response = RegisterResponse {
        status,
        server_hwid,
        server_address,
        text,
    };

    let _ = app_handle.emit("register_request", register_response.clone());

    Json(register_response)
}

async fn add_time_handler(
    Json(payload): Json<AddTimeRequest>,
    app_handle: AppHandle,
    tx: mpsc::Sender<(u64, AppHandle)>
) -> Json<AddTimeResponse> {
    let response = AddTimeResponse {
        status: true,
        text: "Time added successfully".to_string(),
    };

    let _ = tx.send((payload.credits.clone() as u64, app_handle)).await;

    Json(response)
}
