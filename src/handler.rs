use axum::{extract::Path, http::StatusCode, Json};
use serde_json::{json, Value};
use crate::model::Note;

pub async fn healthcheck() -> (StatusCode, Json<Value>) {
    let resp = Json(json!({"data": "OK"}));
    (StatusCode::OK, resp)
}

pub async fn get_notes_list() -> (StatusCode, Json<Value>) {
    let sample_note = Note {
        id: 1,
        title: "Test".to_string(),
        content: "Hello World!".to_string()
    };
    (StatusCode::OK, Json(json!({"data": [
        sample_note
    ]})))
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
