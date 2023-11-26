use crate::handler;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use sqlx::{Pool, Postgres};

pub fn create_api_route(state: Pool<Postgres>) -> Router {
    let api_routes = Router::new()
        .route(
            // GET /notes, POST /notes
            "/notes",
            get(handler::get_notes_list).post(handler::post_note),
        )
        .route(
            // GET /notes/:id, PATCH /notes/:id, DELETE /notes/:id
            "/notes/:id",
            get(handler::get_note_by_id)
                .patch(handler::update_note)
                .delete(handler::delete_note),
        );

    Router::new()
        .route("/healthcheck", get(handler::healthcheck))
        .nest("/api/v1", api_routes) // The routes now would be, e.g: GET /api/v1/notes, GET /api/v1/notes/:id
        .with_state(state)
}
