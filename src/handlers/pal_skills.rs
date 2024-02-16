use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{model::PalSkills, AppState};
use serde_json::json;

pub async fn get_pal_skills(
    Path(name): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let pals_skills_record =
        sqlx::query_as::<_, PalSkills>(r#"SELECT pb.id, pb.name, 
       ps.skills_0_name, ps.skills_1_name, ps.skills_2_name, ps.skills_3_name, ps.skills_4_name, ps.skills_5_name, ps.skills_6_name, 
       ps.skills_0_power, ps.skills_1_power, ps.skills_2_power, ps.skills_3_power, ps.skills_4_power, ps.skills_5_power, ps.skills_6_power, 
       s0.type as skills_0_type, s1.type as skills_1_type, s2.type as skills_2_type, s3.type as skills_3_type, s4.type as skills_4_type, s5.type as skills_5_type, s6.type as skills_6_type, 
       s0.description as skills_0_desc, s1.description as skills_1_desc, s2.description as skills_2_desc, s3.description as skills_3_desc, s4.description as skills_4_desc, 
       s5.description as skills_5_desc, s6.description as skills_6_desc
       FROM public.pals_basic as pb NATURAL JOIN public.pal_skills as ps
       INNER JOIN public.skills as s0 ON s0.name=ps.skills_0_name
       INNER JOIN public.skills as s1 ON s1.name=ps.skills_1_name 
       INNER JOIN public.skills as s2 ON s2.name=ps.skills_2_name 
       INNER JOIN public.skills as s3 ON s3.name=ps.skills_3_name 
       INNER JOIN public.skills as s4 ON s4.name=ps.skills_4_name 
       INNER JOIN public.skills as s5 ON s5.name=ps.skills_5_name 
       INNER JOIN public.skills as s6 ON s6.name=ps.skills_6_name 
       WHERE pb.name = $1"#)
            .bind(name.clone())
            .fetch_one(&data.db)
            .await;

    match pals_skills_record {
        Ok(pal) => {
            let pal_response = serde_json::json!({"status": "success","data": serde_json::json!({
            "pal": pal.name,
            "id": pal.id,
            "skills": {
                "skill_0": {
                    "name": pal.skills_0_name,
                    "power": pal.skills_0_power,
                    "type": pal.skills_0_type,
                    "desc": pal.skills_0_desc
                },
                "skill_1": {
                    "name": pal.skills_1_name,
                    "power": pal.skills_1_power,
                    "type": pal.skills_1_type,
                    "desc": pal.skills_1_desc
                },
                "skill_2": {
                    "name": pal.skills_2_name,
                    "power": pal.skills_2_power,
                    "type": pal.skills_2_type,
                    "desc": pal.skills_2_desc
                },
                "skill_3": {
                    "name": pal.skills_3_name,
                    "power": pal.skills_3_power,
                    "type": pal.skills_3_type,
                    "desc": pal.skills_3_desc
                },
                "skill_4": {
                    "name": pal.skills_4_name,
                    "power": pal.skills_4_power,
                    "type": pal.skills_4_type,
                    "desc": pal.skills_4_desc
                },
                "skill_5": {
                    "name": pal.skills_5_name,
                    "power": pal.skills_5_power,
                    "type": pal.skills_5_type,
                    "desc": pal.skills_5_desc
                },
                "skill_6": {
                    "name": pal.skills_6_name,
                    "power": pal.skills_6_power,
                    "type": pal.skills_6_type,
                    "desc": pal.skills_6_desc
                }
            }
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
