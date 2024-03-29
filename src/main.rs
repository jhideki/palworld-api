mod handlers {
    pub mod basic;
    pub mod pal_drops;
    pub mod pal_info;
    pub mod pal_skills;
    pub mod pal_suitabilities;
}
mod exceptions;
mod model;
mod route;
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
mod body_schema;
use dotenv::dotenv;
use exceptions::ExceptionMap;
use route::create_router;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub struct AppState {
    db: PgPool,
    breed_exceptions: ExceptionMap,
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
            println!("DB connection succeeded");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the db: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    let exception_map = ExceptionMap::new();

    let app = create_router(Arc::new(AppState {
        db: pool.clone(),
        breed_exceptions: exception_map,
    }))
    .layer(cors);

    println!("Server started");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
