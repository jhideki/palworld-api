use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::basic::{breeding_calc, get_pal_basic},
    handlers::pal_drops::get_pal_drops,
    handlers::pal_info::get_pal_info,
    handlers::pal_skills::get_pal_skills,
    handlers::pal_suitabilities::get_pal_suitabilities,
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/pals/:name", get(get_pal_basic))
        .route("/api/skills/:name", get(get_pal_skills))
        .route("/api/pals/breeding_calc", post(breeding_calc))
        .route("/api/pals_info/:name", get(get_pal_info))
        .route("/api/pal_drops/:name", get(get_pal_drops))
        .route("/api/pal_suitabilities/:name", get(get_pal_suitabilities))
        .with_state(app_state)
}
