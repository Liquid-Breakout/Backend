use crate::Backend;  

mod structs;
mod rbxm;

impl Backend {
    pub async fn whitelist_asset(&self, asset_id: u64, user_id_requesting: u64) -> Result<(), Box<dyn std::error::Error>> {
        if !self.user_own_asset_internal(user_id_requesting, asset_id).await? {
            return Err("User does not own asset.".into())
        }
        let item_details = self.fetch_asset_details_internal(asset_id).await?;
        if item_details.is_public_domain.is_none() || !item_details.is_public_domain.unwrap() {
            return Err("Asset is not for sale.".into())
        } else if item_details.asset_type_id.is_none() || item_details.asset_type_id.unwrap() != structs::AssetType::Model {
            return Err("Asset type is not a Model.".into())
        } else if item_details.price_in_robux.is_some() && item_details.price_in_robux.unwrap() > 0 {
            return Err("Asset costs robux.".into())
        }

        self.purchase_asset_internal(asset_id).await?;
        Ok(())
    }

    pub async fn download_asset_bytes(&self, asset_id: u64) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.download_asset_internal(asset_id).await
    }
}

mod internal {
    use surf::{http::Method, RequestBuilder, Response, Url};
    use crate::Backend; 
    use super::structs::{AssetPurchaseReq, ItemDetails};

    const AUTH_URL: &str = "https://auth.roblox.com";
    const ASSETDELIVERY_URL: &str = "https://assetdelivery.roblox.com/v1";
    const ECONOMY_V1_URL: &str = "https://economy.roblox.com/v1";
    const ECONOMY_V2_URL: &str = "https://economy.roblox.com/v2";
    const INVENTORY_URL: &str = "https://inventory.roblox.com/v1";
    const XCSRF_HEADER: &str = "x-csrf-token";

    impl Backend {
        pub(crate) fn construct_request(&self, url: &str, method: Method) -> Result<RequestBuilder, Box<dyn std::error::Error>>  {
            let url = Url::parse(url)?;

            let builder = RequestBuilder::new(method, url)
                .header(XCSRF_HEADER, self.roblox_xcsrf_token.to_owned())
                .header("cookie", self.roblox_cookie.to_owned());

            Ok(builder)
        }

        pub(crate) async fn receive_response(&self, request_builder: RequestBuilder) -> Result<Response, Box<dyn std::error::Error>> {
            match request_builder.send().await {
                Ok(res) => Ok(res),
                Err(e) => Err(e.to_string().into())
            }
        }

        pub(crate) async fn receive_response_as_string(&self, request_builder: RequestBuilder) -> Result<String, Box<dyn std::error::Error>> {
            match request_builder.recv_string().await {
                Ok(result) => Ok(result),
                Err(e) => Err(e.to_string().into())
            }
        }

        pub(crate) async fn receive_response_as_bytes(&self, request_builder: RequestBuilder) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
            match request_builder.recv_bytes().await {
                Ok(result) => Ok(result),
                Err(e) => Err(e.to_string().into())
            }
        }

        pub(crate) async fn receive_response_as_json<T: serde::de::DeserializeOwned>(&self, request_builder: RequestBuilder) -> Result<T, Box<dyn std::error::Error>> {
            match request_builder.recv_json::<T>().await {
                Ok(result) => Ok(result),
                Err(e) => Err(e.to_string().into())
            }
        }

        pub(crate) async fn refresh_xcsrf_token(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            let request_result = self.construct_request(AUTH_URL, Method::Post)?.await?;
    
            let xcsrf = request_result
                .header(XCSRF_HEADER)
                .map(|x| x.as_str().to_string())
                .unwrap();
    
            self.roblox_xcsrf_token = xcsrf;
            Ok(())
        }

        pub(super) async fn download_asset_internal(&self, asset_id: u64) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
            let formatted_url = format!(
                "{}/assetId/{}",
                ASSETDELIVERY_URL,
                asset_id
            );

            let request_result = self.receive_response_as_bytes(self.construct_request(&formatted_url, Method::Get)?).await?;
            Ok(request_result)
        }
    
        pub(super) async fn user_own_asset_internal(&self, user_id: u64, asset_id: u64) -> Result<bool, Box<dyn std::error::Error>> {
            let formatted_url = format!(
                "{}/users/{}/items/Asset/{}/is-owned",
                INVENTORY_URL,
                user_id,
                asset_id
            );
    
            let request_result = self.receive_response_as_string(self.construct_request(&formatted_url, Method::Get)?).await;
            match request_result.unwrap_or(String::new()).parse::<bool>() {
                Ok(res) => Ok(res),
                Err(_) => Ok(false)
            }
        }
    
        pub(super) async fn fetch_asset_details_internal(&self, asset_id: u64) -> Result<ItemDetails, Box<dyn std::error::Error>> {
            let formatted_url = format!(
                "{}/assets/{}/details",
                ECONOMY_V2_URL,
                asset_id
            );
    
            Ok(self.receive_response_as_json::<ItemDetails>(self.construct_request(&formatted_url, Method::Get)?).await?)
        }
    
        pub(super) async fn purchase_asset_internal(&self, asset_id: u64) -> Result<bool, Box<dyn std::error::Error>> {
            let formatted_url = format!(
                "{}/purchases/products/{}",
                ECONOMY_V1_URL,
                asset_id
            );
    
            let request_body = AssetPurchaseReq {
                expected_currency: 1,
                expected_price: 0,
            };
    
            let builder = self.construct_request(&formatted_url, Method::Post)?
                .body_json(&request_body)?;

            let request_result = self.receive_response(builder).await;
            match request_result {
                Ok(res) => Ok(res.status() == 200),
                Err(e) => Err(e)
            }
        }
    }
}