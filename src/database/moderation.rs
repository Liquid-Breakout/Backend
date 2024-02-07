use mongodb::{bson::doc, Collection};
use serde::{ Deserialize, Serialize };
use futures::stream::TryStreamExt;

use crate::Backend;
use crate::utils::datetime_now;

#[derive(Serialize, Deserialize, Debug)]
pub struct BanEntry {
    #[serde(rename = "userId")]
    pub user_id: i64,
    #[serde(rename = "bannedTime")]
    pub banned_time: i64,
    #[serde(rename = "bannedUntil")]
    pub banned_until: i64,
    pub moderator: String,
    pub reason: String
}

impl Backend {
    pub async fn get_ban_collection(&self) -> Result<Vec<BanEntry>, Box<dyn std::error::Error>> {
        let database = self.get_database();

        let collection: Collection<BanEntry> = database.collection("bannedplayers");

        let cursor = collection.find(None, None).await?;
        let result = cursor.try_collect().await?;

        Ok(result)
    }

    pub(crate) async fn find_ban_entry(&self, user_id: i64) -> Result<Option<BanEntry>, Box<dyn std::error::Error>> {
        let database = self.get_database();

        let collection: Collection<BanEntry> = database.collection("bannedplayers");

        let result = collection.find_one(
            doc! { 
                "userId": user_id
            },
            None
        ).await?;

        Ok(result)
    }

    pub async fn ban_player(&self, user_id: i64, duration_in_minutes: i64, moderator: &str, reason: &str) -> Result<(), Box<dyn std::error::Error>> {
        let database = self.get_database();

        let collection: Collection<BanEntry> = database.collection("bannedplayers");

        let time_now: i64 = datetime_now() as i64;
        let banned_until = if duration_in_minutes != -1 { time_now as i64 + (duration_in_minutes * 60)} else { -1 };
        if let Ok(Some(_)) = self.find_ban_entry(user_id).await {
            let update = doc! { "$set": doc! {
                "bannedTime": time_now,
                "bannedUntil": banned_until,
                "moderator": moderator.to_string(),
                "reason": reason.to_string()
            } };

            collection.update_one(doc! { 
                "userId": user_id
            }, update, None).await?;
        } else {
            collection.insert_one(BanEntry {
                user_id: user_id,
                banned_time: time_now,
                banned_until: banned_until,
                moderator: moderator.to_string(),
                reason: reason.to_string()
            }, None).await?;
        }

        Ok(())
    }

    pub async fn unban_player(&self, user_id: i64) -> Result<(), Box<dyn std::error::Error>> {
        let database = self.get_database();

        let collection: Collection<BanEntry> = database.collection("bannedplayers");

        if let Ok(Some(_)) = self.find_ban_entry(user_id).await {
            collection.delete_one(doc! { "userId": user_id }, None).await?;
        }

        Ok(())
    }
}