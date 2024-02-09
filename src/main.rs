use axum::{response::IntoResponse, routing::get, Json, Router};
use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;

pub struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("database_rul must be set ");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection succeeded");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the db: {:?}", err);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/palworldapi", get(palworld_halndler))
        .with_state(Arc::new(AppState { db: pool.clone() }));
    println!("Server started");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn palworld_halndler() -> impl IntoResponse {
    const MESSAGE: &str = "Rust palworld API";
    let json_response = serde_json::json!({
    "status": "success",
    "message":MESSAGE
    });
    Json(json_response)
}
