use serde_derive::{Deserialize, Serialize};

use crate::schema::items;
use crate::schema::lists;
use crate::schema::users;

/// Users

#[derive(Serialize, Debug, Clone, Queryable, Identifiable)]
#[table_name = "users"]
pub struct User {
    pub id: i64,
    pub nick: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable)]
#[table_name = "users"]
pub struct CreateUser {
    pub nick: String,
    pub email: String,
}

/// Lists

#[derive(Serialize, Debug, Clone, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
#[table_name = "lists"]
pub struct List {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub subtitle: String,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "lists"]
pub struct CreateList {
    pub user_id: i64,
    pub title: String,
    pub subtitle: String,
}

/// Items

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
