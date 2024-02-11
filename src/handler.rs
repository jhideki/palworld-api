use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{
    body_schema::BreedData,
    model::{PalBasic, PalBasicResponse, PalBreeding},
    AppState,
};
use serde_json::json;

pub async fn get_pal_handler(
    Path(name): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let pals_basic_record =
        sqlx::query_as::<_, PalBasic>(r#"SELECT * FROM public.pals_basic WHERE name = $1"#)
            .bind(name.clone())
            .fetch_one(&data.db)
            .await;

    match pals_basic_record {
        Ok(pal) => {
            let pal_response = serde_json::json!({"status": "success","data": serde_json::json!({
                        "pal": filter_db_record(&pal).name
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

pub async fn breeding_calc_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<BreedData>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let pal_breeding_record = sqlx::query_as::<_, PalBreeding>(
        r#"SELECT * FROM public.pal_breeding ORDER BY ABS(breeding_power -
        (SELECT CAST ( FLOOR(AVG(breeding_power)) AS INT4) FROM public.pal_breeding WHERE name = $1 OR name = $1)) LIMIT 1;"#,
    )
    .bind(body.father.to_string())
    .bind(body.mother.to_string())
    .fetch_one(&data.db)
    .await;
    match pal_breeding_record {
        Ok(record) => {
            let pal_response = serde_json::json!({"status": "success","data": serde_json::json!({
                        "pal": record.name
            })});
            return Ok(Json(pal_response));
        }
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
            "status":"fail",
            "message": format!("No child found")
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": format!("{:?}",e)})),
            ));
        }
    }
}

fn filter_db_record(pal_record: &PalBasic) -> PalBasicResponse {
    PalBasicResponse {
        id: pal_record.id.to_owned(),
        key: pal_record.key.to_owned(),
        name: pal_record.name.to_owned(),
        wiki: pal_record.wiki.to_owned(),
        image_wiki: pal_record.image_wiki.to_owned(),
    }
}
