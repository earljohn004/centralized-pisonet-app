use axum::{ routing::post, Json, Router };
use serde::{ Deserialize, Serialize };
use tauri::{ AppHandle, Emitter };

#[derive(Deserialize)]
struct RegisterRequest {
    pair_id: String,
    address: String,
    hwid: String,
}

#[derive(Serialize, Clone)]
struct RegisterResponse {
    status: bool,
    server_hwid: String,
    server_address: String,
    text: String,
}

pub async fn start_server(app_handle: AppHandle) {
    // let app = Router::new().route("/api/v1/register", post(register_handler));
    let app = Router::new().route(
        "/api/v1/register",
        axum::routing::post(move |payload| { register_handler(payload, app_handle.clone()) })
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
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
