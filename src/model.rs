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
    use sqlx::PgPool;

    use super::*;

    #[sqlx::test]
    async fn get_one_note_should_pass(pool: PgPool) -> sqlx::Result<()> {
        // Insert single note
        let new_note = CreateNote {
            title: "hello world".to_string(),
            content: "".to_string(),
        };

        let created_note = create_note(&pool, new_note).await?;

        let result = get_one_note(&pool, created_note.id).await;

        if let Ok(note) = result {
            assert_eq!(note.title, "hello world".to_string());
        }

        Ok(())
    }

    #[sqlx::test]
    async fn get_notes_should_pass(pool: PgPool) -> sqlx::Result<()> {
        // Insert single note
        let new_note = CreateNote {
            title: "hello world".to_string(),
            content: "".to_string(),
        };

        let _ = create_note(&pool, new_note).await;

        if let Ok(notes) = get_notes(&pool).await {
            assert_eq!(notes.len(), 1);
        }
        Ok(())
    }

    #[sqlx::test]
    async fn create_note_should_pass(pool: PgPool) -> sqlx::Result<()> {
        let new_note = CreateNote {
            title: "hello world".to_string(),
            content: "".to_string(),
        };

        let result = create_note(&pool, new_note).await;

        if let Ok(note) = result {
            assert_eq!(note.title, "hello world".to_string());
        }

        Ok(())
    }

    // #[tokio::test]
    // async fn test_should_fail() {
    // assert_eq!(1,2);
    // }
}
