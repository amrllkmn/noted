use crate::model::{self, create_note, CreateNote};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use sqlx::{Pool, Postgres};

pub async fn healthcheck() -> (StatusCode, Json<Value>) {
    let resp = Json(json!({"data": "OK"}));
    (StatusCode::OK, resp)
}

pub async fn get_notes_list() -> (StatusCode, Json<Value>) {
    if let Ok(sample_note) = model::get_one_note().await {
        (StatusCode::OK, Json(json!({ "data": [sample_note] })))
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
        "data": "Note not found"})),
        )
    }
}

pub async fn post_note(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<CreateNote>,
) -> (StatusCode, Json<Value>) {
    let result = create_note(&pool, payload).await;
    match result {
        Ok(note) => (StatusCode::CREATED, Json(json! {note})),
        Err(sqlx::Error::Protocol(err)) => (
            // If protocol error, then it's a unique constraint violation
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({ "message": err })),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json! ({"message":"Something went wrong"})),
        ),
    }
}

pub async fn get_note(Path(_id): Path<String>) -> (StatusCode, Json<Value>) {
    todo!()
}

pub async fn update_note(Path(_id): Path<String>) -> (StatusCode, Json<Value>) {
    todo!()
}

pub async fn delete_note(Path(_id): Path<String>) -> (StatusCode, Json<Value>) {
    todo!()
}
