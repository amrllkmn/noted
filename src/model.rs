use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Error, FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Note {
    id: Uuid,
    title: String,
    content: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNote {
    title: String,
    content: String,
}

impl Note {
    fn new(title: String, content: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            content,
            created_at: now,
            updated_at: now,
        }
    }
}

pub async fn get_one_note(pool: &PgPool, note_id: Uuid) -> Result<Note, Error> {
    let result = query_as::<_, Note>("SELECT * FROM notes WHERE id = $1")
        .bind(note_id)
        .fetch_one(pool)
        .await;

    match result {
        Ok(note) => Ok(note),
        Err(err) => Err(err),
    }
}

pub async fn get_notes(pool: &PgPool) -> Result<Vec<Note>, Error> {
    let result = query_as::<_, Note>("SELECT * FROM notes")
        .fetch_all(pool)
        .await;
    match result {
        Ok(notes) => Ok(notes),
        Err(err) => Err(err),
    }
}

pub async fn create_note(pool: &PgPool, new_note: CreateNote) -> Result<Note, Error> {
    let note = Note::new(new_note.title, new_note.content);
    let result = query(
        r#"
        INSERT INTO notes (id, title, content, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(note.id)
    .bind(&note.title)
    .bind(&note.content)
    .bind(note.created_at)
    .bind(note.updated_at)
    .execute(pool)
    .await;

    match result {
        Ok(_) => Ok(note),
        Err(Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(Error::Protocol("The title must be unique".to_string()))
            } else {
                Err(Error::Database(db_err))
            }
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod test {
    use sqlx::postgres::PgPoolOptions;

    use super::*;
    use dotenv::dotenv;
    use std::env;

    #[tokio::test]
    async fn get_one_note_should_pass() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL env");
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .unwrap_or_else(|err| {
                eprintln!("Failed to connect to database: {:?}", err);
                panic!("Database connection error")
            });

        let result = get_one_note(
            &pool,
            Uuid::parse_str("e26ccf37-aaa7-4031-a67b-d16a3f990632").unwrap(),
        )
        .await;

        assert!(result.is_ok());

        if let Ok(note) = result {
            assert_eq!(note.title, "hello world".to_string());
        }
    }

    #[tokio::test]
    async fn get_notes_should_pass() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL env");
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .unwrap();
        if let Ok(notes) = get_notes(&pool).await {
            assert_eq!(notes.len(), 2);
        }
    }

    #[tokio::test]
    async fn create_note_should_pass() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL env");
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .unwrap();

        let new_note = CreateNote {
            title: "hello world".to_string(),
            content: "".to_string(),
        };

        if let Ok(note) = create_note(&pool, new_note).await {
            assert_eq!(note.title, "hello world".to_string());
        }
    }

    // #[tokio::test]
    // async fn test_should_fail() {
    // assert_eq!(1,2);
    // }
}
