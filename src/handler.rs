use crate::model::{self, CreateNote};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn healthcheck() -> (StatusCode, Json<Value>) {
    let resp = Json(json!({"data": "OK"}));
    (StatusCode::OK, resp)
}

pub async fn get_notes_list(State(pool): State<Pool<Postgres>>) -> (StatusCode, Json<Value>) {
    if let Ok(sample_note) = model::get_notes(&pool).await {
        (StatusCode::OK, Json(json! {sample_note}))
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "Something went wrong"})),
        )
    }
}

pub async fn post_note(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<CreateNote>,
) -> (StatusCode, Json<Value>) {
    let result = model::create_note(&pool, payload).await;
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

pub async fn get_note_by_id(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<Value>) {
    if let Ok(note_id) = Uuid::parse_str(id.as_str()) {
        let result = model::get_one_note(&pool, note_id).await;
        match result {
            Ok(note) => (StatusCode::OK, Json(json! {note})),
            Err(sqlx::Error::RowNotFound) => (
                StatusCode::NOT_FOUND,
                Json(json!({"message": "Note not found"})),
            ),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Something went wrong"})),
            ),
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "The ID provided is not valid UUID"})),
        )
    }
}

pub async fn update_note(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<String>,
    Json(payload): Json<CreateNote>,
) -> (StatusCode, Json<Value>) {
    if let Ok(note_id) = Uuid::parse_str(id.as_str()) {
        let result = model::update_note(&pool, payload, note_id).await;

        match result {
            Ok(_) => (StatusCode::OK, Json(json!({"message": "Note updated"}))),
            Err(sqlx::Error::RowNotFound) => (
                StatusCode::NOT_FOUND,
                Json(json!({"message": "Note not found"})),
            ),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Something went wrong"})),
            ),
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "The ID provided is not valid UUID"})),
        )
    }
}

pub async fn delete_note(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<Value>) {
    if let Ok(note_id) = Uuid::parse_str(id.as_str()) {
        let result = model::delete_note(&pool, note_id).await;
        match result {
            Ok(_) => (StatusCode::OK, Json(json!({"message": "Note deleted"}))),
            Err(sqlx::Error::RowNotFound) => (
                StatusCode::NOT_FOUND,
                Json(json!({"message": "Note not found"})),
            ),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Something went wrong"})),
            ),
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "The ID provided is not valid UUID"})),
        )
    }
}
