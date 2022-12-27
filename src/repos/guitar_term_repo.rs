use anyhow::Error;
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::{Client, Collection};

use crate::utils::db::get_db_name;

pub struct GuitarTermRepository {
    collection: Collection<Document>,
}

impl GuitarTermRepository {
    pub fn new(client: &Client, environment: &str) -> GuitarTermRepository {
        let db = client.database(&get_db_name(&environment));
        let feeds = db.collection::<Document>("guitarterms");

        GuitarTermRepository { collection: feeds }
    }

    pub async fn get_all(&self) -> Result<Vec<String>, Error> {
        let find_options = mongodb::options::FindOptions::builder()
            .projection(doc! {"_id": 1})
            .build();

        let cursor = self.collection.find(None, find_options).await?;
        let guitar_terms: Vec<Document> = cursor.try_collect().await?;

        let ids: Vec<String> = guitar_terms
            .iter()
            .map(|doc| doc.get_str("_id").unwrap().to_string())
            .collect();

        Ok(ids)
    }
}
