use parking_lot::RwLock;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

pub type Items = HashMap<String, i32>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {
    pub name: String,
    pub quantity: i32,
}

#[derive(Clone)]
pub struct Store {
    pub list: Arc<RwLock<Items>>,
}

impl Store {
    pub fn new() -> Self {
        println!("Creating localstore...");
        Store {
            list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
