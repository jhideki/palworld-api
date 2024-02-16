use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{model::PalSuitabilities, AppState};
use serde_json::json;
pub async fn get_pal_suitabilities(
    Path(name): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let pal_suitabilities_record = sqlx::query_as::<_, PalSuitabilities>(
        r#"SELECT name, id, suitability_0, suitability_1, suitability_2, suitability_3, suitability_4, suitability_5, suitability_6, level_0, level_1, level_2, level_3, level_4, level_5, level_6 
        FROM public.pal_suitabilities NATURAL JOIN public.pals_basic WHERE name = $1"#,
    )
    .bind(name.clone())
    .fetch_one(&data.db)
    .await;

    match pal_suitabilities_record {
        Ok(pal) => {
            let pal_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "name": pal.name,
                "id": pal.id,
                "suitability_0": pal.suitability_0,
                "suitability_1": pal.suitability_1,
                "suitability_2": pal.suitability_2,
                "suitability_3": pal.suitability_3,
                "suitability_4": pal.suitability_4,
                "suitability_5": pal.suitability_5,
                "suitability_6": pal.suitability_6,
                "level_0": pal.level_0,
                "level_1": pal.level_1,
                "level_2": pal.level_2,
                "level_3": pal.level_3,
                "level_4": pal.level_4,
                "level_5": pal.level_5,
                "level_6": pal.level_6
            })});
            return Ok(Json(pal_response));
        }
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
            "status":"fail",
            "message": format!("Pal with name: {} not found", name)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": format!("{:?}",e)})),
            ));
        }
    };
}
