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


    async fn update_list(&self, list: &List) -> TodoResult<()> {
        let collection = self.db.collection::<List>("lists");
        collection.replace_one(doc! { "name": &list.name }, list, None).await?;
        Ok(())
    }

    async fn create_list(&self, list: List) -> TodoResult<()> {
        let collection = self.db.collection::<List>("lists");
        collection.insert_one(list, None).await?;
        Ok(())
    }

    pub async fn new() -> TodoResult<Self> {
        let uri = std::env::var("MONGODB_URI").map_err(|_| TodoError::InvalidInput("MONGODB_URI must be set".to_string()))?;
        let mut client_options = ClientOptions::parse(uri).await?;
        client_options.app_name = Some("Todo App".to_string());
        let client = Client::with_options(client_options)?;
        let db = client.database("todo_app");
        Ok(Self { db })
    }

    pub async fn get_lists(&self) -> TodoResult<Vec<List>> {
        let collection = self.db.collection::<List>("lists");
        let mut cursor = collection.find(None, None).await?;
        let mut lists = Vec::new();
        while let Some(list) = cursor.try_next().await? {
            lists.push(list);
        }
        Ok(lists)
    }

    pub async fn get_list(&self, name: &str) -> TodoResult<List> {
        let collection = self.db.collection::<List>("lists");
        collection.find_one(doc! { "name": name }, None).await?
            .ok_or_else(|| TodoError::ListNotFound(name.to_string()))
    }

    pub async fn add_item(&self, list_name: &str, item: Item) -> TodoResult<()> {
        let collection = self.db.collection::<List>("lists");
        let result = collection.update_one(
            doc! { "name": list_name },
            doc! {
                "$setOnInsert": { "name": list_name },
                "$push": { "items": bson::to_document(&item).map_err(|err| TodoError::DatabaseError(err.into()))? }
            },
            mongodb::options::UpdateOptions::builder().upsert(true).build(),
        ).await?;
    
        if result.matched_count == 0 && result.upserted_id.is_none() {
            return Err(TodoError::ListNotFound(list_name.to_string()));
        }
        Ok(())
    }

    pub async fn update_item_status(&self, list_name: &str, item_number: usize, completed: bool) -> TodoResult<()> {
        let collection = self.db.collection::<List>("lists");
        let result = collection.update_one(
            doc! { "name": list_name },
            doc! { "$set": { format!("items.{}.completed", item_number - 1): completed } },
            None,
        ).await?;

        if result.matched_count == 0 {
            return Err(TodoError::ListNotFound(list_name.to_string()));
        }
        if result.modified_count == 0 {
            return Err(TodoError::ItemNotFound(format!("Item {} in list {}", item_number, list_name)));
        }
        Ok(())
    }

    pub async fn remove_item(&self, list_name: &str, item_number: usize) -> TodoResult<()> {
        let collection = self.db.collection::<List>("lists");
        let result = collection.update_one(
            doc! { "name": list_name },
            doc! { "$unset": { format!("items.{}", item_number - 1): "" } },
            None,
        ).await?;

        if result.matched_count == 0 {
            return Err(TodoError::ListNotFound(list_name.to_string()));
        }
        if result.modified_count == 0 {
            return Err(TodoError::ItemNotFound(format!("Item {} in list {}", item_number, list_name)));
        }

        collection.update_one(
            doc! { "name": list_name },
            doc! { "$pull": { "items": null } },
            None,
        ).await?;
        Ok(())
    }

    pub async fn remove_list(&self, list_name: &str) -> TodoResult<()> {
        let collection = self.db.collection::<List>("lists");
        let result = collection.delete_one(doc! { "name": list_name }, None).await?;
        if result.deleted_count == 0 {
            return Err(TodoError::ListNotFound(list_name.to_string()));
        }
        Ok(())
    }

    pub async fn remove_all_lists(&self) -> TodoResult<()> {
        let collection = self.db.collection::<List>("lists");
        collection.delete_many(doc! {}, None).await?;
        Ok(())
    }

}


