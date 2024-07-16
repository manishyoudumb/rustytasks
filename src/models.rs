use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]

pub(crate) struct List {
    pub name: String,
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Item {
    pub description: String,
    pub completed: bool,
}