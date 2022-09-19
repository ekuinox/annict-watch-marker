use anyhow::{Result, ensure};
use super::AnnictClient;


impl AnnictClient {
    pub async fn status(&self, work_id: i64, kind: &str) -> Result<()> {
        let status_code = reqwest::Client::new()
            .post("https://api.annict.com/v1/me/statuses")
            .query(&[
                ("access_token", self.access_token.as_str()),
                ("work_id", work_id.to_string().as_str()),
                ("kind", kind),
            ])
            .send()
            .await?
            .status();
        ensure!(status_code.is_success());
        Ok(())
    }
}
