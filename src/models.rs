use crate::schema::items;
use crate::schema::lists;

use serde_derive::Serialize;

#[derive(Serialize, Debug, Clone, Queryable, Identifiable)]
#[table_name = "lists"]
pub struct List {
    pub id: i64,
    pub title: String,
    pub info: String,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "lists"]
pub struct CreateList {
    pub title: String,
    pub info: String,
}

#[derive(Serialize, Debug, Clone, Queryable, Identifiable, Associations)]
#[belongs_to(List)]
#[table_name = "items"]
pub struct Item {
    pub id: i64,
    pub list_id: i64,
    pub title: String,
    pub amount: i32,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "items"]
pub struct CreateItem {
    pub list_id: i64,
    pub title: String,
    pub amount: i32,
}
