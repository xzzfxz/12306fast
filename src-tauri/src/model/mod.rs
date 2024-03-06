use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Station {
    #[serde(rename = "jianPin")]
    pub jian_pin: String,
    pub name: String,
    pub id: String,
    #[serde(rename = "areaCode")]
    pub area_code: Option<String>,
    #[serde(rename = "quanPin")]
    pub quan_pin: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpResult<T> {
    pub data: T,
    pub httpstatus: u32,
    pub messages: String,
    pub status: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TrainData {
    pub result: Vec<String>,
    pub map: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TrainInfo {
    #[serde(rename = "trainId")]
    pub train_id: String,
    #[serde(rename = "trainName")]
    pub train_name: String,
    #[serde(rename = "startStationId")]
    pub start_station_id: String,
    #[serde(rename = "startStationName")]
    pub start_station_name: String,
    #[serde(rename = "endStationId")]
    pub end_station_id: String,
    #[serde(rename = "endStationName")]
    pub end_station_name: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    // 历时
    pub duration: String,
    // 商务座/特等座
    #[serde(rename = "businessSeat")]
    pub business_seat: String,
    // 一等座
    #[serde(rename = "firstClassSeat")]
    pub first_class_seat: String,
    // 二等座
    #[serde(rename = "secondClassSeat")]
    pub second_class_seat: String,
    // 高级软卧
    #[serde(rename = "superSoftSleeper")]
    pub super_soft_sleeper: String,
    // 软卧/一等卧
    #[serde(rename = "softSleeper")]
    pub soft_sleeper: String,
    // 硬卧/二等卧
    #[serde(rename = "hardSleeper")]
    pub hard_sleeper: String,
    // 硬座
    #[serde(rename = "hardSeat")]
    pub hard_seat: String,
    #[serde(rename = "noSeat")]
    pub no_seat: String,
}
