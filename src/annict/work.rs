use super::AnnictClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorksResponse {
    pub works: Vec<Work>,
    pub total_count: i64,
    pub next_page: Value,
    pub prev_page: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Work {
    pub id: i64,
    pub title: String,
    pub title_kana: String,
    pub title_en: String,
}

impl AnnictClient {
    pub async fn get_works_by_title(&self, title: &str) -> Result<GetWorksResponse> {
        let json = reqwest::Client::new()
            .get("https://api.annict.com/v1/works")
            .query(&[
                ("access_token", self.access_token.as_str()),
                ("filter_title", title),
            ])
            .send()
            .await?
            .json()
            .await?;
        Ok(json)
    }
}
