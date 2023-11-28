use serde::{Deserialize, Serialize};
use sqlx::{error::DatabaseError, query, query_as, Error, FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Note {
    id: Uuid,
    title: String,
    content: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

pub async fn update_note(
    pool: &PgPool,
    updated_note: CreateNote,
    note_id: Uuid,
) -> Result<u64, Error> {
    let now = chrono::Utc::now();
    let result = query(
        r#"
        UPDATE notes
        SET title = $1, content = $2, updated_at = $3
        WHERE id =$4
        "#,
    )
    .bind(&updated_note.title)
    .bind(&updated_note.content)
    .bind(now)
    .bind(note_id)
    .execute(pool)
    .await;

    if let Ok(rows) = result {
        if rows.rows_affected() == 0 {
            Err(Error::RowNotFound)
        } else {
            Ok(rows.rows_affected())
        }
    } else {
        Err(Error::Protocol("Something went wrong".to_string()))
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
    async fn get_one_note_should_fail_if_not_found(pool: PgPool) -> sqlx::Result<()> {
        // note_id doesn't exist in the test database
        let note_id = Uuid::parse_str("6f845dfe-fa30-45eb-89fb-aa15de682a26").unwrap();
        let result = get_one_note(&pool, note_id).await;

        assert!(result.is_err_and(|err| matches!(err, sqlx::Error::RowNotFound)));
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

    #[sqlx::test]
    async fn create_two_notes_with_same_title_should_fail(pool: PgPool) -> sqlx::Result<()> {
        let new_note = CreateNote {
            title: "hello world".to_string(),
            content: "".to_string(),
        };

        let second_note = CreateNote {
            title: "hello world".to_string(),
            content: "".to_string(),
        };

        let _ = create_note(&pool, new_note).await?;

        let duplicate_note = create_note(&pool, second_note).await;

        assert!(duplicate_note.is_err_and(|err| matches!(err, sqlx::Error::Protocol(_))));
        Ok(())
    }

    #[sqlx::test]
    async fn update_a_note_should_pass(pool: PgPool) -> sqlx::Result<()> {
        let new_note = CreateNote {
            title: "hello world".to_string(),
            content: "".to_string(),
        };

        let updated_note = CreateNote {
            title: "hello world".to_string(),
            content: "Added some content".to_string(),
        };

        let created_note = create_note(&pool, new_note).await?;

        let update_result = update_note(&pool, updated_note, created_note.id).await?;

        assert_eq!(update_result, 1);

        Ok(())
    }

    #[sqlx::test]
    async fn update_bad_note_id_should_fail(pool: PgPool) -> sqlx::Result<()> {
        let random_generated_uuid = Uuid::new_v4();

        let new_note = CreateNote {
            title: "hello world".to_string(),
            content: "".to_string(),
        };

        let note = create_note(&pool, new_note).await?;

        let bad_note = CreateNote {
            title: note.title,
            content: "This doesn't exists".to_string(),
        };

        let updated_result = update_note(&pool, bad_note, random_generated_uuid).await;
        assert!(updated_result.is_err_and(|err| matches!(err, Error::RowNotFound)));
        Ok(())
    }
}
