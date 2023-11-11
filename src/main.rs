use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;

async fn healthcheck() -> (StatusCode, Json<Value>) {
    let resp = Json(json!({"data": "OK"}));
    (StatusCode::OK, resp)
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = env::var("PORT_NUMBER").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    println!("Listening on {}", addr);
    let app = Router::new().route("/healthcheck", get(healthcheck));
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
