use reqwest;
use crate::id_converter::IDConverter;
mod id_converter;

pub struct Backend {
    roblox_cookie: String,
    id_generator: IDConverter
}

impl Backend {
    pub fn whitelist_asset(asset_id: u64, user_id_requesting: u64) {

    }

    pub fn get_shareable_id(id: String) -> String {
        String::new()
    }

    pub fn get_number_id(id: String) -> String {
        String::new()
    }
}