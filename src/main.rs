mod handler;
mod model;
mod routes;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let database_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL env");

    let state = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Listening on {}", addr);

    sqlx::migrate!().run(&state).await?;

    let app = routes::create_api_route(state);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
