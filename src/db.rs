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

    pub async fn pull_changes(&self) -> TodoResult<()> {
        let collection = self.db.collection::<List>("lists");
        let mut cursor = collection.find(None, None).await?;

        while let Some(list) = cursor.try_next().await? {
            let existing_list = self.get_list(&list.name).await;
            match existing_list {
                Ok(existing) => {
                    if existing.name != list.name || existing.items.len() != list.items.len() {
                        self.update_list(&list).await?;
                    } else {
                        for (existing_item, new_item) in existing.items.iter().zip(list.items.iter()) {
                            if existing_item.description != new_item.description || existing_item.completed != new_item.completed {
                                self.update_list(&list).await?;
                                break;
                            }
                        }
                    }
                },
                Err(TodoError::ListNotFound(_)) => {
                    self.create_list(list).await?;
                },
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }



}