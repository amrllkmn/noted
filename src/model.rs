use axum::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize, Serialize)]
pub struct Note {
    pub id: u32,
    pub title: String,
    pub content: String
}

pub async fn get_one_note() -> Result<Note, Error> {

    let sample_note = Note {
        id: 1,
        title: "Test".to_string(),
        content: "Hello World!".to_string()
    };

    Ok(sample_note)
}

#[cfg(test)]

mod test {
    use super::get_one_note;


    #[tokio::test]
    async fn test_should_pass() {
        if let Ok(note)  =  get_one_note().await {
            assert_eq!(note.content, "Hello World!".to_string());    
        }
    }

    // #[tokio::test]
    // async fn test_should_fail() {
        // assert_eq!(1,2);
    // }
}
