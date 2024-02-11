use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{breeding_calc_handler, get_pal_handler},
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/pals/:name", get(get_pal_handler))
        .route("/api/pals/breeding_calc", post(breeding_calc_handler))
        .with_state(app_state)
}
