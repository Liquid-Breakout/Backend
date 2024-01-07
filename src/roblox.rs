use reqwest::{Client, header};

const AUTH_URL: &str = "https://auth.roblox.com/";
const XCSRF_HEADER: &str = "x-csrf-token";

pub struct RobloxWrapper {
    cookie: String,
    xcsrf_token: String
}
impl RobloxWrapper {
    pub async fn new(cookie: String) -> Self {
        let cookie_value = format!(".ROBLOSECURITY={}", cookie);
        Self { cookie: cookie_value, xcsrf_token: "".to_string() }
    }

    fn prepare_headers(&self) -> header::HeaderMap {
        let mut reqwest_headers = header::HeaderMap::new();
        let mut xcsrf_header = header::HeaderValue::from_static(self.xcsrf_token.as_str());
        reqwest_headers.insert(XCSRF_HEADER, xcsrf_header);
        let mut cookie_header = header::HeaderValue::from_static(self.cookie.as_str());
        cookie_header.set_sensitive(true);
        reqwest_headers.insert("cookie", cookie_header);

        reqwest_headers
    }

    pub async fn refresh_xcsrf_token(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let request_result = Client::new()
            .post(AUTH_URL)
            .headers(self.prepare_headers())
            .send()
            .await?;

        let xcsrf = request_result
            .headers()
            .get(XCSRF_HEADER)
            .map(|x| x.to_str().unwrap().to_string())
            .unwrap();

        self.xcsrf_token = xcsrf;
        Ok(())
    }

    pub async fn user_own_asset(&self, user_id: u64, asset_id: u64) -> Result<bool, Box<dyn std::error::Error>> {
        let formatted_url = format!(
            "https://inventory.roblox.com/v1/users/{}/items/Asset/{}/is-owned",
            user_id,
            asset_id
        );

        let request_result = reqwest::Client::new()
            .get(formatted_url)
            .headers(self.prepare_headers())
            .send()
            .await?;

        match request_result.text().await.unwrap_or(String::new()).parse::<bool>() {
            Ok(res) => Ok(res),
            Err(_) => Ok(false)
        }
    }
}