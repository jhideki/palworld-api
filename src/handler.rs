use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::intoResponse,
    Json,
};
use std::sync::Arc;

use crate::{model::PalBasic, AppState};
use serde_json::json;

pub async fn get_pal_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let pals_basic_record = sqlx::query_as!(
        PalBasic,
        r#"select * from public.pals_basic where name = ?"#,
        name as String
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let json_response = match pals_basic_record {
        Some(record) => {
            serde_json::json!({
            "status": "success",
            "pal":record
            })
        }
        None => {
            serde_json::json!({
            "status: fail",
            "message": "No record found with this name"
            })
        }
    };
}
