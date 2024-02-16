use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{model::PalDrops, AppState};
use serde_json::json;
pub async fn get_pal_drops(
    Path(name): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let pal_drops_record= sqlx::query_as::<_, PalDrops>(
        r#"SELECT name, id, drop_0, drop_1, drop_2, drop_3 FROM public.pal_drops NATURAL JOIN public.pals_basic WHERE name = $1"#,
    )
    .bind(name.clone())
    .fetch_one(&data.db)
    .await;

    match pal_drops_record {
        Ok(pal) => {
            let pal_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "name": pal.name,
                "id": pal.id,
                "drop 0": pal.drop_0,
                "drop 1": pal.drop_1,
                "drop 2": pal.drop_2,
                "drop 3": pal.drop_3,
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
