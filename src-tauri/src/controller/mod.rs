use crate::model::Station;
use crate::service;

/// 获取常用站点
#[tauri::command]
pub async fn get_common_station() -> Vec<Station> {
    let list = match service::get_common_stations().await {
        Ok(list) => return list,
        Err(e) => {
            println!("获取常用站点发生了错误: {:#?}", e);
            vec![]
        }
    };
    list
}

/// 获取所有站点
#[tauri::command]
pub async fn get_all_station() -> Vec<Station> {
    let list = match service::get_all_station().await {
        Ok(list) => return list,
        Err(e) => {
            println!("获取所有站点发生了错误: {:#?}", e);
            vec![]
        }
    };
    list
}
