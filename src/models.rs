use super::schema::lists;

use diesel::{Insertable, Queryable};
use serde::Deserialize;
use serde::Serialize;

#[derive(Insertable, Serialize, Deserialize, Debug, Clone, Queryable)]
#[table_name = "lists"]
pub struct CreateListDTO {
    pub id: i32,
    pub title: String,
    pub info: String,
    pub published: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Queryable)]
pub struct ListDTO {
    pub title: String,
    pub info: String,
    pub published: bool,
}
