pub struct RobloxWrapper {
    cookie: String,
    reqwest_client: reqwest::Client
}
impl RobloxWrapper {
    pub fn new(cookie: String, reqwest_client: reqwest::Client) -> Self {
        Self { cookie: cookie, reqwest_client: reqwest_client }
    }
    pub async fn user_own_asset(&self, user_id: u64, asset_id: u64) -> Result<bool, Box<dyn std::error::Error>> {
        let formatted_url = format!(
            "https://inventory.roblox.com/v1/users/{}/items/Asset/{}/is-owned",
            user_id,
            asset_id
        );

        let request_result = self
            .reqwest_client
            .get(formatted_url)
            .send()
            .await?;

        match request_result.text().await.unwrap_or(String::new()).parse::<bool>() {
            Ok(res) => Ok(res),
            Err(_) => Ok(false)
        }
    }
}