mod network;

use crate::model::Station;
use anyhow::Result;

// 格式化站点字符串为list
async fn format_station_list(all_str: &str, is_common: bool) -> Result<Vec<Station>> {
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
        if is_common {
            let jian_pin = info_arr[0].trim().to_string();
            let name = info_arr[1].trim().to_string();
            let id = info_arr[2].trim().to_string();
            let station = Station {
                jian_pin,
                name,
                id,
                area_code: None,
                quan_pin: None,
                jian_name: None,
            };
            station_list.push(station);
        }
    }
    Ok(station_list)
}

// 获取常用站点
pub async fn get_common_stations() -> Result<Vec<Station>> {
    let name_str = network::fetch_favorite_names().await?;
    let list = format_station_list(&name_str, true).await?;
    Ok(list)
}
