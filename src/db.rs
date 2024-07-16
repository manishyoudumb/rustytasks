use futures::TryStreamExt;
use mongodb::bson;
use mongodb::{Client, Database as MongoDatabase, bson::doc, options::ClientOptions};
use crate::models::{List, Item};
use crate::error::{TodoError, TodoResult};


pub struct Database {
    db: MongoDatabase,
}