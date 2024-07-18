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
