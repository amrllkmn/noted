use axum::{extract::Path, http::StatusCode, Json};
use serde_json::{json, Value};
use crate::model;

pub async fn healthcheck() -> (StatusCode, Json<Value>) {
    let resp = Json(json!({"data": "OK"}));
    (StatusCode::OK, resp)
}

pub async fn get_notes_list() -> (StatusCode, Json<Value>) {
    
    if let Ok(sample_note) = model::get_one_note().await {
        
        (StatusCode::OK, Json(json!({"data": [
            sample_note
        ]})))
    } else {
        (StatusCode::NOT_FOUND, Json(json!({
        "data": "Note not found"})))
    }
}

pub async fn post_note() -> (StatusCode, Json<Value>) {
    todo!()
}

pub async fn get_note(Path(id): Path<String>) -> (StatusCode, Json<Value>) {
    todo!()
}

pub async fn update_note(Path(id): Path<String>) -> (StatusCode, Json<Value>) {
    todo!()
}

pub async fn delete_note(Path(id): Path<String>) -> (StatusCode, Json<Value>) {
    todo!()
}

