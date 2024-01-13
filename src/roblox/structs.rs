use serde::{Deserialize, Serialize};

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
Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
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
pub struct AssetPurchaseReq {
    pub expected_currency: u64,
    pub expected_price: u64
}