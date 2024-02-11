use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct PalBasic {
    pub id: i64,
    pub key: String,
    pub name: String,
    pub wiki: String,
    pub image_wiki: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PalBasicResponse {
    pub id: i64,
    pub key: String,
    pub name: String,
    pub wiki: String,
    pub image_wiki: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct PalBreeding {
    pub name: String,
    pub breeding_power: i64,
    pub type_0: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_1: Option<String>,
}
