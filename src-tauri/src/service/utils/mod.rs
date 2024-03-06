use std::collections::HashMap;

use crate::model::{Station, TrainInfo};
use anyhow::Result;

// 格式化站点字符串为list
pub async fn format_station_list(all_str: &str, is_common: bool) -> Result<Vec<Station>> {
    let str_arr: Vec<&str> = all_str.split("=").collect();
    let station_str = str_arr[1].trim();
    // 删除单绰引号和分号
    let info_str: String = station_str
        .chars()
        .filter(|&c| !['\'', ';'].contains(&c))
        .collect();
    // 删除第一个字符
    let info_str = &info_str[1..info_str.len() - 1];
    let station_str_arr: Vec<&str> = info_str.split("@").collect();
    let mut station_list: Vec<Station> = vec![];
    for info in station_str_arr {
        let info_arr: Vec<&str> = info.split("|").collect();
        let jian_pin = info_arr[0].trim().to_string();
        let name = info_arr[1].trim().to_string();
        let id = info_arr[2].trim().to_string();
        let mut quan_pin: Option<String> = None;
        let mut area_code: Option<String> = None;
        if !is_common {
            quan_pin = Some(info_arr[3].trim().to_string());
            area_code = Some(info_arr[5].trim().to_string());
        }
        let station = Station {
            jian_pin,
            name,
            id,
            area_code,
            quan_pin,
        };
        station_list.push(station);
    }
    Ok(station_list)
}

// 处理座位信息
pub fn format_seat_info(seat: &str) -> String {
    if seat.is_empty() {
        "-".to_string()
    } else if seat == "无" {
        "候补".to_string()
    } else {
        seat.to_string()
    }
}

// 格式化车次信息
pub fn format_train_list(
    list: Vec<String>,
    map: HashMap<String, String>,
) -> Result<Vec<TrainInfo>> {
    let mut result: Vec<TrainInfo> = vec![];
    for info in list {
        let arr: Vec<&str> = info.split("|").collect();
        let train_info = TrainInfo {
            train_id: arr[2].trim().to_string(),
            train_name: arr[3].trim().to_string(),
            start_station_id: arr[4].trim().to_string(),
            start_station_name: map.get(&arr[4].trim().to_string()).unwrap().to_string(),
            end_station_id: arr[5].trim().to_string(),
            end_station_name: map.get(&arr[5].trim().to_string()).unwrap().to_string(),
            start_time: arr[8].trim().to_string(),
            end_time: arr[9].trim().to_string(),
            duration: arr[10].trim().to_string(),
            business_seat: format_seat_info(arr[32].trim()),
            first_class_seat: format_seat_info(arr[31].trim()),
            second_class_seat: format_seat_info(arr[30].trim()),
            super_soft_sleeper: format_seat_info(arr[21].trim()),
            soft_sleeper: format_seat_info(arr[23].trim()),
            hard_sleeper: format_seat_info(arr[28].trim()),
            hard_seat: format_seat_info(arr[29].trim()),
            no_seat: format_seat_info(arr[26].trim()),
        };
        result.push(train_info);
    }
    Ok(result)
}
