use std::sync::Arc;
use mongodb::options::{ClientOptions, ResolverConfig};
use tokio::sync::Mutex;
use serde_json;
use mongodb::{Client, Database as MongoDatabase};
use crate::models::{List, Item};
use crate::error::{TodoError, TodoResult};
use std::time::SystemTime;


pub struct Database {
    local_db: Arc<Mutex<serde_json::Value>>,
    pub remote_db: MongoDatabase,
    dirty: Arc<Mutex<bool>>,
    #[allow(dead_code)]
    last_modified: Arc<Mutex<SystemTime>>,
}

impl Database {

    pub async fn create_list(&self, list_name: &str) -> TodoResult<()> {
        let mut local_db = self.local_db.lock().await;
        local_db[list_name] = serde_json::Value::Array(Vec::new());
        *self.dirty.lock().await = true;
        self.save_local_db(&local_db).await
    }

    pub async fn new() -> TodoResult<Self> {
        let uri = std::env::var("MONGODB_URI")
            .map_err(|_| TodoError::ConfigError("MONGODB_URI must be set".to_string()))?;
    
        let mut options = ClientOptions::parse_with_resolver_config(&uri, ResolverConfig::cloudflare()).await?;
        options.app_name = Some("Todo App".to_string());
    
        let client = Client::with_options(options)?;
        let remote_db = client.database("todo_app");
    
        let local_db = Self::load_local_db().await?;
        let local_db = Arc::new(Mutex::new(local_db));
    
        let dirty = Arc::new(Mutex::new(false));
        let last_modified = Arc::new(Mutex::new(SystemTime::now()));
    
        Ok(Self {
            local_db,
            remote_db,
            dirty,
            last_modified,
        })
    }

    async fn load_local_db() -> TodoResult<serde_json::Value> {
        let data = tokio::fs::read_to_string("local_db.json").await.unwrap_or_else(|_| "{}".to_string());
        Ok(serde_json::from_str(&data)?)
    }
    

    pub async fn add_item(&self, list_name: &str, item: Item) -> TodoResult<()> {
        let mut local_db = self.local_db.lock().await;
        let list = local_db.get_mut(list_name)
            .and_then(|v| v.as_array_mut())
            .ok_or_else(|| TodoError::ListNotFound(list_name.to_string()))?;
        list.push(serde_json::to_value(item)?);
        *self.dirty.lock().await = true;
        self.save_local_db(&local_db).await?;
        println!("Local database updated and marked as dirty.");
        Ok(())
    }

    pub async fn get_lists(&self) -> TodoResult<Vec<List>> {
        let local_db = self.local_db.lock().await;
        let lists: Vec<List> = local_db.as_object()
            .ok_or_else(|| TodoError::DatabaseError("Invalid local database format".into()))?
            .iter()
            .map(|(name, items)| List {
                name: name.clone(),
                items: serde_json::from_value(items.clone()).unwrap_or_default(),
            })
            .collect();
        Ok(lists)
    }

    pub async fn get_list(&self, name: &str) -> TodoResult<List> {
        let local_db = self.local_db.lock().await;
        let items = local_db.get(name)
            .ok_or_else(|| TodoError::ListNotFound(name.to_string()))?;
        Ok(List {
            name: name.to_string(),
            items: serde_json::from_value(items.clone())?,
        })
    }

    pub async fn update_item_status(&self, list_name: &str, item_number: usize, completed: bool) -> TodoResult<()> {
        let mut local_db = self.local_db.lock().await;
        let list = local_db.get_mut(list_name)
            .and_then(|v| v.as_array_mut())
            .ok_or_else(|| TodoError::ListNotFound(list_name.to_string()))?;

        if item_number == 0 || item_number > list.len() {
            return Err(TodoError::ItemNotFound(format!("Item {} in list {}", item_number, list_name)));
        }

        let item = list.get_mut(item_number - 1)
            .and_then(|v| v.as_object_mut())
            .ok_or_else(|| TodoError::DatabaseError("Invalid local database format".into()))?;

        item.insert("completed".to_string(), serde_json::Value::Bool(completed));
        *self.dirty.lock().await = true;
        self.save_local_db(&local_db).await?;
        Ok(())
    }

    pub async fn remove_item(&self, list_name: &str, item_number: usize) -> TodoResult<()> {
        let mut local_db = self.local_db.lock().await;
        let list = local_db.get_mut(list_name)
            .and_then(|v| v.as_array_mut())
            .ok_or_else(|| TodoError::ListNotFound(list_name.to_string()))?;

        if item_number == 0 || item_number > list.len() {
            return Err(TodoError::ItemNotFound(format!("Item {} in list {}", item_number, list_name)));
        }

        list.remove(item_number - 1);
        *self.dirty.lock().await = true;
        self.save_local_db(&local_db).await?;
        Ok(())
    }

    pub async fn remove_list(&self, list_name: &str) -> TodoResult<()> {
        let mut local_db = self.local_db.lock().await;
        if local_db.as_object_mut().unwrap().remove(list_name).is_none() {
            return Err(TodoError::ListNotFound(list_name.to_string()));
        }
        *self.dirty.lock().await = true;
        self.save_local_db(&local_db).await?;
        Ok(())
    }

    pub async fn remove_all_lists(&self) -> TodoResult<()> {
        let mut local_db = self.local_db.lock().await;
        *local_db = serde_json::Value::Object(serde_json::Map::new());
        *self.dirty.lock().await = true;
        self.save_local_db(&local_db).await?;
        Ok(())
    }

    pub async fn set_dirty(&self, value: bool) {
        *self.dirty.lock().await = value;
    }

    pub async fn update_local_db(&self, new_db: serde_json::Value) -> TodoResult<()> {
        let mut local_db = self.local_db.lock().await;
        *local_db = new_db;
        self.save_local_db(&local_db).await
    }

    async fn save_local_db(&self, local_db: &serde_json::Value) -> TodoResult<()> {
        let data = serde_json::to_string_pretty(&local_db)?;
        tokio::fs::write("local_db.json", data).await?;
        Ok(())
    }

    
    
    pub async fn get_local_db(&self) -> TodoResult<serde_json::Value> {
        let local_db = self.local_db.lock().await;
        Ok(local_db.clone())
    }
    
    

    pub async fn update_last_modified(&self) {
        *self.last_modified.lock().await = SystemTime::now();
    }

}


// #[tokio::test]
// async fn test_get_local_db() {
//     let db = Database::new().await.unwrap();
//     let result = db.get_local_db().await;
//     assert!(result.is_ok());
//     // Wow, it actually returned something. What a miracle!
// }

// #[tokio::test]
// async fn test_update_last_modified() {
//     let db = Database::new().await.unwrap();
//     let before = *db.last_modified.lock().await;
//     db.update_last_modified().await;
//     let after = *db.last_modified.lock().await;
//     assert!(after > before);
//     // Time moves forward. Who would've thought?
// }
