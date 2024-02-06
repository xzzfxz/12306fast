use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Station {
    #[serde(rename = "jianPin")]
    pub jian_pin: String,
    pub name: String,
    pub id: String,
    #[serde(rename = "areaCode")]
    pub area_code: Option<u32>,
    #[serde(rename = "quanPin")]
    pub quan_pin: Option<String>,
    #[serde(rename = "jianName")]
    pub jian_name: Option<String>,
}
