use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct PalBasic {
    pub id: i64,
    pub key: i64,
    pub name: String,
    pub wiki: String,
    pub image_wiki: String,
}
