use futures::TryStreamExt;
use mongodb::bson;
use mongodb::options::ResolverConfig;
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
        let mut options = ClientOptions::parse_with_resolver_config(&uri, ResolverConfig::cloudflare()).await?;
        
        options.app_name = Some("Todo App".to_string());
        let client = Client::with_options(options)?;
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


#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use dotenv::from_filename;

    #[tokio::test]
async fn test_push_and_pull_changes() -> TodoResult<()> {
    from_filename(".env").ok();
    let db1 = Database::new().await?;
    let db2 = Database::new().await?;

    // Clear any existing data
    db1.remove_all_lists().await?;

    // Create unique test data
    db1.add_item("Keir", Item { description: "Finish quantum algorithm".to_string(), completed: false }).await?;
    db1.add_item("Maverick", Item { description: "Test hypersonic jet".to_string(), completed: true }).await?;
    db1.add_item("Cyborg", Item { description: "Upgrade neural interface".to_string(), completed: false }).await?;

    db1.push_changes().await?;
    db2.pull_changes().await?;

    // Verify pulled changes
    let lists = db2.get_lists().await?;
    assert_eq!(lists.len(), 3, "Should have 3 lists after pull");

    let keir_list = db2.get_list("Keir").await?;
    assert_eq!(keir_list.items.len(), 1, "Keir's list should have 1 item");
    assert_eq!(keir_list.items[0].description, "Finish quantum algorithm");
    assert_eq!(keir_list.items[0].completed, false);

    // Modify data in db2
    db2.update_item_status("Keir", 1, true).await?;
    db2.remove_item("Cyborg", 1).await?;
    db2.add_item("AI", Item { description: "Develop sentient AI".to_string(), completed: false }).await?;

    db2.push_changes().await?;
    db1.pull_changes().await?;

    // Verify updated data in db1
    let updated_lists = db1.get_lists().await?;
    assert_eq!(updated_lists.len(), 4, "Should have 4 lists after updates");

    let updated_keir_list = db1.get_list("Keir").await?;
    assert_eq!(updated_keir_list.items.len(), 1, "Updated Keir's list should have 1 item");
    assert_eq!(updated_keir_list.items[0].completed, true);

    let cyborg_list = db1.get_list("Cyborg").await?;
    assert_eq!(cyborg_list.items.len(), 0, "Cyborg's list should be empty");

    let ai_list = db1.get_list("AI").await?;
    assert_eq!(ai_list.items.len(), 1, "AI list should have 1 item");
    assert_eq!(ai_list.items[0].description, "Develop sentient AI");

    // Clean up
    db1.remove_all_lists().await?;

    Ok(())
}

}    