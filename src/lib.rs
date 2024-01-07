use id_converter::IDConverter;
use roblox::RobloxWrapper;

mod id_converter;
mod roblox;
mod utils;
pub struct Backend {
    rbx_client: RobloxWrapper,
    id_generator: IDConverter
}

impl Backend {
    pub fn new(roblox_cookie: String, id_generator_alphabets: Vec<String>) -> Self {
        if id_generator_alphabets.len() < 2 {
            panic!("ID Generator must have at least 2 alphabets.");
        }
        let rbx_client = RobloxWrapper::new(roblox_cookie);
        let id_generator = IDConverter::new(&id_generator_alphabets[0], &id_generator_alphabets[1]);
        Self { rbx_client: rbx_client, id_generator: id_generator }
    } 
    //

    pub async fn whitelist_asset(&self, asset_id: u64, user_id_requesting: u64) -> Result<(), Box<dyn std::error::Error>> {
        if !self.rbx_client.user_own_asset(user_id_requesting, asset_id).await.unwrap(){ //why are you so subspace_tripmine
           // Err("User does not own asset.".into())
           //Err(|e: std::string::ParseError | e.to_string())
        }
        Ok(())
    }

    pub fn get_shareable_id(&self, id: String) -> Result<String, Box<dyn std::error::Error>> {
        let id = id.parse::<u64>();
        if id.is_ok() {
            let converted_id = self.id_generator.to_short(id.unwrap());
            if converted_id.is_ok() {
                Ok(converted_id.unwrap())
            } else {
                Err(converted_id.unwrap_err())
            }
        } else {
            Err("ID cannot be converted to integer.".into())
        }
    }

    pub fn get_number_id(&self, id: String) -> Result<u64, Box<dyn std::error::Error>> {
        let converted_id = self.id_generator.to_number(id);
        if converted_id.is_ok() {
            Ok(converted_id.unwrap())
        } else {
            Err(converted_id.unwrap_err())
        }
    }
}