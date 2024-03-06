use anyhow::Result;

use crate::model::{HttpResult, TrainData};

/// 获取常用站点
pub async fn fetch_favorite_names() -> Result<String> {
    let url = "https://kyfw.12306.cn/otn/resources/js/framework/favorite_name.js";
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;
    let body = res.text().await?;
    Ok(body)
}

/// 获取所有站点
pub async fn fetch_all_names() -> Result<String> {
    let url = "https://kyfw.12306.cn/otn/resources/js/framework/station_name.js";
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;
    let body = res.text().await?;
    Ok(body)
}

/// 根据站点和日期查询车次信息
pub async fn fetch_left_ticket(
    date: String,
    from_station: String,
    end_station: String,
) -> Result<HttpResult<TrainData>> {
    let url = format!("https://kyfw.12306.cn/otn/leftTicket/query?leftTicketDTO.train_date={date}&leftTicketDTO.from_station={from}&leftTicketDTO.to_station={to}&purpose_codes=ADULT", date = date, from = from_station, to = end_station);
    let cookie = format!("_jc_save_fromDate={}", date);
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(reqwest::header::COOKIE, cookie,)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
        .send()
        .await?;
    let body = res.json::<HttpResult<TrainData>>().await?;
    Ok(body)
}
