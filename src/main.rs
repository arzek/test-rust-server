use axum::{
    extract::Query,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_http::cors::CorsLayer;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    timestamp: u64,
}

#[derive(Serialize)]
struct HelloResponse {
    message: String,
    name: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct EchoRequest {
    message: String,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "OK".to_string(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    })
}

async fn hello(Query(params): Query<HashMap<String, String>>) -> Json<HelloResponse> {
    let name = params.get("name").cloned();
    Json(HelloResponse {
        message: "Hello from Rust!".to_string(),
        name,
    })
}

async fn echo(Json(payload): Json<EchoRequest>) -> Json<EchoRequest> {
    Json(payload)
}

async fn not_found() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({
            "error": "Not found",
            "message": "The requested resource was not found"
        })),
    )
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "🚀 Super Fast Rust Server!" }))
        .route("/health", get(health))
        .route("/hello", get(hello))
        .route("/echo", post(echo))
        .fallback(not_found)
        .layer(CorsLayer::permissive());

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!("🚀 Server running on http://0.0.0.0:{}", port);
    axum::serve(listener, app).await.unwrap();
}