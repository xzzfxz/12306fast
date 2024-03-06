mod network;
mod utils;

use crate::model::{Station, TrainInfo};
use anyhow::Result;

/// 获取常用站点
pub async fn get_common_stations() -> Result<Vec<Station>> {
    let name_str = network::fetch_favorite_names().await?;
    let list = utils::format_station_list(&name_str, true).await?;
    Ok(list)
}

/// 获取所有站点
pub async fn get_all_station() -> Result<Vec<Station>> {
    let name_str = network::fetch_all_names().await?;
    let list = utils::format_station_list(&name_str, false).await?;
    Ok(list)
}

/// 根据站点和日期查询车次信息
pub async fn get_left_ticket(
    date: String,
    from_station: String,
    end_station: String,
) -> Result<Vec<TrainInfo>> {
    let res = network::fetch_left_ticket(date, from_station, end_station).await?;
    if res.status != true {
        return Ok(vec![]);
    }
    let list = utils::format_train_list(res.data.result, res.data.map)?;
    Ok(list)
}
