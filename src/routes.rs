use crate::handler;
use axum::{routing::get, Router};
use http::{HeaderValue, Method};
use sqlx::{Pool, Postgres};
use std::env;
use tower_http::cors::CorsLayer;

pub fn create_api_route(state: Pool<Postgres>) -> Router {
    let front_end_url: String = env::var("FRONT_END_URL").expect("Missing FRONT_END_URL");
    let allowed_origin = front_end_url.parse::<HeaderValue>().unwrap();
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([
            Method::GET,
            Method::PATCH,
            Method::POST,
            Method::DELETE,
            Method::OPTIONS,
        ])
        // allow requests from any origin
        .allow_origin(allowed_origin);
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
        .layer(cors) // This raises an error
}
