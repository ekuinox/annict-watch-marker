use super::AnnictClient;
use anyhow::{ensure, Result};

impl AnnictClient {
    pub async fn record(&self, episode_id: i64) -> Result<()> {
        let episode_id = episode_id.to_string();
        let status_code = reqwest::Client::new()
            .post("https://api.annict.com/v1/me/records")
            .query(&[
                ("access_token", self.access_token.as_str()),
                ("episode_id", episode_id.as_str()),
            ])
            .send()
            .await?
            .status();
        ensure!(status_code.is_success(), status_code);
        Ok(())
    }
}
