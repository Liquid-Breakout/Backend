use mongodb::{Client, options::ClientOptions};

use id_converter::IDConverter;
use roblox::RobloxWrapper;

mod id_converter;
mod roblox;
mod utils;
pub mod database;
pub struct Backend {
    rbx_client: RobloxWrapper,
    id_generator: IDConverter,
    mongo_client: Option<Client>
}

impl Backend {
    pub fn new(roblox_cookie: String, id_generator_alphabets: Vec<String>) -> Self {
        if id_generator_alphabets.len() < 2 {
            panic!("ID Generator must have at least 2 alphabets.");
        }
        let rbx_client = RobloxWrapper::new(roblox_cookie);
        let id_generator = IDConverter::new(&id_generator_alphabets[0], &id_generator_alphabets[1]);

        Self { rbx_client: rbx_client, id_generator: id_generator, mongo_client: None }
    } 
    
    pub async fn connect_mongodb(&mut self, mongodb_url: String) -> Result<(), Box<dyn std::error::Error>> {
        let mongo_options = ClientOptions::parse(mongodb_url).await?;
        let mongo_client = Client::with_options(mongo_options)?;

        self.mongo_client = Some(mongo_client);
        Ok(())
    }

    pub async fn whitelist_asset(&self, asset_id: u64, user_id_requesting: u64) -> Result<(), Box<dyn std::error::Error>> {
        if !self.rbx_client.user_own_asset(user_id_requesting, asset_id).await.unwrap() { //why are you so subspace_tripmine
            // Err("User does not own asset.".into())
            //Err(|e: std::string::ParseError | e.to_string())
        }
        let item_details = self.rbx_client.fetch_asset_details(asset_id).await?;
        if item_details.is_public_domain.is_none() || !item_details.is_public_domain.unwrap() {
            panic!("Asset is not for sale.")
        } else if item_details.price_in_robux.is_none() || item_details.asset_type_id.unwrap() != roblox::AssetType::Model {
            panic!("Asset type is not a Model.")
        } else if item_details.price_in_robux.is_some() && item_details.price_in_robux.unwrap() > 0 {
            panic!("Asset costs robux.")
        }

        self.rbx_client.purchase_asset(asset_id).await?;
        Ok(())
    }

    pub fn get_shareable_id(&self, id: String) -> Result<String, Box<dyn std::error::Error>> {
        let parsed_id = id.parse::<u64>();
        match parsed_id {
            Ok(i) => self.id_generator.to_short(i.into()),
            Err(_) => panic!("ID cannot be converted into integer.")
        }
    }

    pub fn get_number_id(&self, id: String) -> Result<u128, Box<dyn std::error::Error>> {
        self.id_generator.to_number(id)
    }
}