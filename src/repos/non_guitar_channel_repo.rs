use anyhow::Error;
use mongodb::bson::{doc, DateTime, Document};
use mongodb::{Client, Collection};

use crate::utils::db::get_db_name;

pub struct NonGuitarChannelRepository {
    collection: Collection<Document>,
}

impl NonGuitarChannelRepository {
    pub fn new(client: &Client, environment: &str) -> NonGuitarChannelRepository {
        let db = client.database(&get_db_name(&environment));
        let channels = db.collection::<Document>("nonguitarchannels");

        NonGuitarChannelRepository {
            collection: channels,
        }
    }

    pub async fn exists(&self, channel_id: &str) -> Result<bool, Error> {
        let result = self
            .collection
            .count_documents(doc! { "_id": channel_id }, None)
            .await?;

        Ok(result > 0)
    }

    pub async fn upsert(&self, channel_id: &str) {
        let update_options = mongodb::options::UpdateOptions::builder()
            .upsert(true)
            .build();

        self.collection
            .update_one(
                doc! {"_id": channel_id},
                doc! {"$set": {"_id": channel_id, "decisionMadeAt": DateTime::now()}},
                update_options,
            )
            .await
            .unwrap();
    }
}
