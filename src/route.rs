use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{handler::get_pal_handler, AppState};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/pals/:id", get(get_pal_handler))
        .with_state(app_state)
}
