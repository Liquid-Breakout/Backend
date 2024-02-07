use serde::{Deserialize, Serialize};

#[derive(
Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum AssetType {
    #[default]
    Image = 1,
    Audio = 3,
    Mesh = 4,
    Lua = 5,
    Model = 10,
    Decal = 13
}

#[derive(
Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum CreatorType {
    #[default]
    User = 1,
    Group = 2,
}

#[derive(
Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
pub struct Creator {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "HasVerifiedBadge")]
    pub has_verified_badge: bool,
    #[serde(rename = "CreatorType")]
    pub creator_type: CreatorType,
    #[serde(rename = "CreatorTargetId")]
    pub target_id: i64,
    #[serde(rename = "Name")]
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemDetails {
    #[serde(rename = "AssetId")]
    pub id: i64,
    #[serde(rename = "TargetId")]
    pub target_id: i64,
    #[serde(rename = "ProductId")]
    pub product_id: i64,
    #[serde(rename = "AssetTypeId")]
    pub asset_type_id: Option<AssetType>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Creator")]
    pub creator: Creator,
    #[serde(rename = "PriceInRobux")]
    pub price_in_robux: Option<u64>,
    #[serde(rename = "CollectibleItemId")]
    pub collectible_item_id: Option<String>,
    #[serde(rename = "IsForSale")]
    pub is_for_sale: Option<bool>,
    #[serde(rename = "IsPublicDomain")]
    pub is_public_domain: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetPurchaseReq {
    #[serde(rename = "expectedCurrency")]
    pub expected_currency: u64,
    #[serde(rename = "expectedPrice")]
    pub expected_price: u64
}