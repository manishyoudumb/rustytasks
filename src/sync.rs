use crate::db::Database;
use crate::error::TodoResult;
use crate::models::List;
use futures::TryStreamExt;

pub async fn push(db: &Database) -> TodoResult<()> {
    println!("Initiating push process...");
    let local_db = db.get_local_db().await?;

    let collection = db.remote_db.collection::<List>("lists");

    // Clear existing data in the collection
    collection.drop(None).await?;

    // Insert each list separately
    for (name, items) in local_db.as_object().unwrap() {
        let list = List {
            name: name.clone(),
            items: serde_json::from_value(items.clone())?,
        };
        collection.insert_one(list, None).await?;
    }

    println!("Local data successfully pushed to 'lists' collection in 'todo_app' database.");
    db.set_dirty(false).await;
    db.update_last_modified().await;

    Ok(())
}



pub async fn pull(db: &Database) -> TodoResult<()> {
    let collection = db.remote_db.collection::<List>("lists");
    let mut cursor = collection.find(None, None).await?;

    let mut new_local_db = serde_json::Map::new();

    while let Some(list) = cursor.try_next().await? {
        new_local_db.insert(list.name.clone(), serde_json::to_value(list.items)?);
    }

    db.update_local_db(serde_json::Value::Object(new_local_db)).await?;
    db.set_dirty(false).await;
    println!("Changes pulled successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Item;
    use mongodb::bson::doc;

    #[tokio::test]
    async fn test_push_and_pull() {
        // Create a test database
        let db = Database::new().await.unwrap();

        // Prepare test data
        let test_list = List {
            name: "Test List".to_string(),
            items: vec![
                Item {
                    description: "Task 1".to_string(),
                    completed: false,
                },
                Item {
                    description: "Task 2".to_string(),
                    completed: true,
                },
            ],
        };

        // Update local database
        let mut local_db = serde_json::Map::new();
        local_db.insert(
            test_list.name.clone(),
            serde_json::to_value(test_list.items.clone()).unwrap(),
        );
        db.update_local_db(serde_json::Value::Object(local_db))
            .await
            .unwrap();

        // Test push
        push(&db).await.unwrap();

        // Verify data in remote database
        let collection = db.remote_db.collection::<List>("lists");
        let remote_list = collection
            .find_one(doc! { "name": "Test List" }, None)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(remote_list.name, test_list.name);
        assert_eq!(remote_list.items.len(), test_list.items.len());
        for (remote_item, test_item) in remote_list.items.iter().zip(test_list.items.iter()) {
            assert_eq!(remote_item.description, test_item.description);
            assert_eq!(remote_item.completed, test_item.completed);
        }

        // Clear local database
        db.update_local_db(serde_json::Value::Object(serde_json::Map::new()))
            .await
            .unwrap();

        // Test pull
        pull(&db).await.unwrap();

        // Verify data in local database
        let local_db = db.get_local_db().await.unwrap();
        let pulled_list = local_db.get("Test List").unwrap();
        assert_eq!(
            pulled_list,
            &serde_json::to_value(test_list.items).unwrap()
        );
    }
}
