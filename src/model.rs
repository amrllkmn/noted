use axum::Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub content: String,
}

pub async fn get_one_note() -> Result<Note, Error> {
    let sample_note = Note {
        id: Uuid::new_v4(),
        title: "Test".to_string(),
        content: "Hello World!".to_string(),
    };

    Ok(sample_note)
}

pub async fn get_notes() -> Result<Vec<Note>, Error> {
    let notes = vec![Note {
        id: Uuid::new_v4(),
        title: "Test".to_string(),
        content: "Hello World!".to_string(),
    }];
    Ok(notes)
}

#[cfg(test)]

mod test {
    use super::*;

    #[tokio::test]
    async fn get_one_note_should_pass() {
        if let Ok(note) = get_one_note().await {
            assert_eq!(note.content, "Hello World!".to_string());
        }
    }

    #[tokio::test]
    async fn get_notes_should_pass() {
        if let Ok(notes) = get_notes().await {
            assert_eq!(notes.len(), 1);
        }
    }

    // #[tokio::test]
    // async fn test_should_fail() {
    // assert_eq!(1,2);
    // }
}
