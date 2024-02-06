use crate::model::Station;
use crate::service;

#[tauri::command]
pub async fn get_common_station() -> Vec<Station> {
    let list = match service::get_common_stations().await {
        Ok(list) => return list,
        Err(e) => {
            println!("发生了错误: {:#?}", e);
            vec![]
        }
    };
    list
}
