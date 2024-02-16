use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct PalBasic {
    pub id: i64,
    pub key: String,
    pub name: String,
    pub wiki: String,
    pub image_wiki: String,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct PalSkills {
    pub id: i64,
    pub name: String,
    pub skills_0_name: String,
    pub skills_1_name: String,
    pub skills_2_name: String,
    pub skills_3_name: String,
    pub skills_4_name: String,
    pub skills_5_name: String,
    pub skills_6_name: String,
    pub skills_0_power: i64,
    pub skills_1_power: i64,
    pub skills_2_power: i64,
    pub skills_3_power: i64,
    pub skills_4_power: i64,
    pub skills_5_power: i64,
    pub skills_6_power: i64,
    pub skills_0_type: String,
    pub skills_1_type: String,
    pub skills_2_type: String,
    pub skills_3_type: String,
    pub skills_4_type: String,
    pub skills_5_type: String,
    pub skills_6_type: String,
    pub skills_0_desc: String,
    pub skills_1_desc: String,
    pub skills_2_desc: String,
    pub skills_3_desc: String,
    pub skills_4_desc: String,
    pub skills_5_desc: String,
    pub skills_6_desc: String,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct PalSuitabilities {
    pub name: String,
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suitability_0: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suitability_1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suitability_2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suitability_3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suitability_4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suitability_5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suitability_6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_0: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_1: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_2: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_3: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_4: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_5: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_6: Option<i64>,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct PalDrops {
    pub name: String,
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_0: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_3: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct PalInfo {
    pub name: String,
    pub id: i64,
    pub type_0: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_1: Option<String>,
    pub genus: String,
    pub rarity: i32,
    pub price: i64,
    pub size: String,
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
