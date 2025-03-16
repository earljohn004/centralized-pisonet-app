use axum::{ routing::{ get, post }, Json, Router };
use serde::{ Deserialize, Serialize };

#[derive(Deserialize)]
struct RegisterRequest {
    pair_id: String,
    address: String,
    hwid: String,
}

#[derive(Serialize)]
struct RegisterResponse {
    status: bool,
    server_hwid: String,
    server_address: String,
    text: String,
}

pub async fn start_server() {
    let app = Router::new().route("/api/v1/register", post(register_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn register_handler(Json(payload): Json<RegisterRequest>) -> Json<RegisterResponse> {
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

    Json(RegisterResponse {
        status,
        server_hwid,
        server_address,
        text,
    })
}
