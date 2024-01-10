use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use crate::utils;

const AUTH_URL: &str = "https://auth.roblox.com";
const ECONOMY_V1_URL: &str = "https://economy.roblox.com/v1";
const ECONOMY_V2_URL: &str = "https://economy.roblox.com/v2";
const INVENTORY_URL: &str = "https://inventory.roblox.com/v1";
const XCSRF_HEADER: &str = "x-csrf-token";

#[derive(
Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum ItemType {
    #[default]
    Asset,
    Bundle,
}

#[derive(
Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum AssetType {
    #[default]
    Image,
    Audio,
    Mesh,
    Lua,
    Model,
    Decal
}
impl AssetType {
    pub(crate) fn as_u8(&self) -> u8 {
        match self {
            Self::Image => 1,
            Self::Audio => 3,
            Self::Mesh => 4,
            Self::Lua => 5,
            Self::Model => 10,
            Self::Decal => 13
        }
    }
}

#[derive(
Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum CreatorType {
    #[default]
    User,
    Group,
}
impl CreatorType {
    pub(crate) fn as_u8(&self) -> u8 {
        match self {
            Self::User => 1,
            Self::Group => 2,
        }
    }
}

#[derive(
Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
#[serde(rename_all = "PascalCase")]
pub struct Creator {
    pub id: u64,
    pub has_verified_badge: bool,
    pub creator_type: CreatorType,
    pub creator_target_id: u64,
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ItemDetails {
    pub id: u64,
    pub target_id: u64,
    pub product_id: u64,
    pub asset_type_id: Option<AssetType>,
    pub name: String,
    pub description: String,
    pub creator: Creator,
    pub price_in_robux: Option<u64>,
    pub collectible_item_id: Option<String>,
    pub is_for_sale: Option<bool>,
    pub is_public_domain: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct AssetPurchaseReq {
    pub expected_currency: u64,
    pub expected_price: u64
}

pub struct RobloxWrapper {
    cookie: String,
    xcsrf_token: String
}
impl RobloxWrapper {
    #[allow(unused_must_use)]
    pub fn new(cookie: String) -> Self {
        let cookie_value = format!(".ROBLOSECURITY={}", cookie);
        let mut wrapper_self = Self { cookie: cookie_value, xcsrf_token: "".to_string() };
        wrapper_self.refresh_xcsrf_token();

        wrapper_self
    }

    fn prepare_headers(&self) -> header::HeaderMap {
        let mut reqwest_headers = header::HeaderMap::new();

        // send help
        let xcsrf_header = header::HeaderValue::from_static(utils::string_to_static_str(self.xcsrf_token.to_owned()));
        reqwest_headers.insert(XCSRF_HEADER, xcsrf_header);
        let mut cookie_header = header::HeaderValue::from_static(utils::string_to_static_str(self.cookie.to_owned()));
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
            "{}/users/{}/items/Asset/{}/is-owned",
            INVENTORY_URL,
            user_id,
            asset_id
        );

        let request_result = Client::new()
            .get(formatted_url)
            .headers(self.prepare_headers())
            .send()
            .await?;

        match request_result.text().await.unwrap_or(String::new()).parse::<bool>() {
            Ok(res) => Ok(res),
            Err(_) => Ok(false)
        }
    }

    pub async fn fetch_asset_details(&self, asset_id: u64) -> Result<ItemDetails, Box<dyn std::error::Error>> {
        let formatted_url = format!(
            "{}/assets/{}/details",
            ECONOMY_V2_URL,
            asset_id
        );

        let request_result = Client::new()
            .get(formatted_url)
            .headers(self.prepare_headers())
            .send()
            .await?;

        Ok(request_result.json::<ItemDetails>().await?)
    }

    pub async fn purchase_asset(&self, asset_id: u64) -> Result<(), Box<dyn std::error::Error>> {
        let formatted_url = format!(
            "{}/purchases/products/{}",
            ECONOMY_V1_URL,
            asset_id
        );

        let request_body = AssetPurchaseReq {
            expected_currency: 1,
            expected_price: 0,
        };

        Client::new()
            .post(formatted_url)
            .headers(self.prepare_headers())
            .json(&request_body)
            .send()
            .await?;

        Ok(())
    }
}