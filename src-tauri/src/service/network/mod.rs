use anyhow::Result;

/// 获取常用站点
pub async fn fetch_favorite_names() -> Result<String> {
    let url = "https://kyfw.12306.cn/otn/resources/js/framework/favorite_name.js";
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;
    let body = res.text().await?;
    Ok(body)
}
