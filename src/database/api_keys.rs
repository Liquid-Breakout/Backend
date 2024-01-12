use mongodb::{bson::doc, Collection};
use serde::{ Deserialize, Serialize };

use crate::Backend;
use crate::IDConverter;
use crate::utils::datetime_now;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
    value: String,
    assign_owner: String,
    associated_discord_user: String,
    enabled: bool,
    time_created: u128,
}

impl Backend {
    pub(super) async fn generate_api_key(&self) -> Result<String, Box<dyn std::error::Error>> {
        let database = self.get_database();
        let api_keys_collection: Collection<ApiKey> = database.collection("apiKeys");
        
        // my reaction when rust
        let api_key_generator: IDConverter = IDConverter::new(
            &"qwertyuiopasdfghjklzxcvbnm0192837465".to_string(),
            &"5432189076".to_string()
        );

        let doc_count: u128 = api_keys_collection.count_documents(None, None).await?.into();
        api_key_generator.to_short(doc_count * 8 + datetime_now() * 2)
    }

    pub async fn get_api_key_entry(&self, api_key: &str) -> Result<ApiKey, Box<dyn std::error::Error>> {
        let database = self.get_database();

        let api_keys_collection: Collection<ApiKey> = database.collection("apiKeys");

        let result = api_keys_collection.find_one(
            doc! { 
                "value": api_key.to_string()
            },
            None
        ).await?;

        match result {
            Some(doc) => Ok(doc),
            None => panic!("API Key does not exist in entries.")
        }
    }

    pub async fn api_key_entry_exist(&self, api_key: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let result = self.get_api_key_entry(api_key).await;

        match result {
            Ok(_) => Ok(true),
            Err(_) => Ok(false)
        }
    }

    pub async fn create_api_key_entry(&self) -> Result<(), Box<dyn std::error::Error>> {
        let database = self.get_database();
        
        let api_keys_collection: Collection<ApiKey> = database.collection("apiKeys");
        let new_api_key = self.generate_api_key().await?;

        let doc = ApiKey {
            value: new_api_key,
            assign_owner: "None".to_string(),
            associated_discord_user: "None".to_string(),
            enabled: true,
            time_created: datetime_now()
        };
        api_keys_collection.insert_one(doc, None).await?;
        Ok(())
    }

    pub fn delete_api_key_entry(&self, api_key: &str) {
        self.get_database();
    }

    pub fn search_api_key_entries_with_roblox_id(&self, roblox_id: u64) {
        self.get_database();
    }

    pub fn search_api_key_entries_with_discord_id(&self, discord_id: u64) {
        self.get_database();
    }   
}