use crate::schema::lists;

use serde_derive::Serialize;

#[derive(Serialize, Debug, Clone, Queryable)]
pub struct ListDTO {
    pub id: i64,
    pub title: String,
    pub info: String,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "lists"]
pub struct CreateListDTO {
    pub title: String,
    pub info: String,
}
