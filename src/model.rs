use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize, Serialize)]
pub struct Note {
    pub id: u32,
    pub title: String,
    pub content: String
}
