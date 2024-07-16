use anyhow::Ok;
use futures::TryStreamExt;
use mongodb::bson;
use mongodb::{Client, Database as MongoDatabase, bson::doc, options::ClientOptions};
use crate::models::{List, Item};
use crate::error::{TodoError, TodoResult};


pub struct Database {
    db: MongoDatabase,
}

impl Database {
    
    pub async fn push_changes(&self) -> TodoResult<()> {
        let collection = self.db.collection::<List>("lists");
        let local_lists = self.get_lists().await?;
        
        for list in local_lists {
            collection.replace_one(
                doc! { "name": &list.name },
                &list,
                mongodb::options::ReplaceOptions::builder().upsert(true).build(),
            ).await?;
        }
        
        Ok(())

    }

    



}