use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{model::PalInfo, AppState};
use serde_json::json;
pub async fn get_pal_info(
    Path(name): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let pal_info_record = sqlx::query_as::<_, PalInfo>(
        r#"SELECT name, id, type_0, type_1, genus, rarity, price, size  FROM public.pal_info NATURAL JOIN public.pals_basic WHERE name = $1"#,
    )
    .bind(name.clone())
    .fetch_one(&data.db)
    .await;

    match pal_info_record {
        Ok(pal) => {
            let pal_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "name": pal.name,
                "id": pal.id,
                "type_0": pal.type_0,
                "type_1": pal.type_1,
                "genus": pal.genus,
                "rarity": pal.rarity,
                "price": pal.price,
                "size": pal.size
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
