use super::AnnictClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEpisodesResponse {
    pub episodes: Vec<Episode>,
    #[serde(rename = "total_count")]
    pub total_count: i64,
    #[serde(rename = "next_page")]
    pub next_page: Value,
    #[serde(rename = "prev_page")]
    pub prev_page: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    pub id: i64,
    pub number: Option<f64>,
    pub title: String,
}

impl AnnictClient {
    pub async fn get_episodes_by_work_id(&self, work_id: i64) -> Result<GetEpisodesResponse> {
        let work_id = work_id.to_string();
        let json = reqwest::Client::new()
            .get("https://api.annict.com/v1/episodes")
            .query(&[
                ("access_token", self.access_token.as_str()),
                ("filter_work_id", &work_id),
            ])
            .send()
            .await?
            .json()
            .await?;
        Ok(json)
    }
}
