use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

use crate::errors::{ApiError, ErrorType};
use crate::models::{CreateItem, Item};
use crate::models::{CreateList, List};

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

pub struct DBManager {
    connection: PooledPg,
}

impl DBManager {
    pub fn new(connection: PooledPg) -> DBManager {
        DBManager { connection }
    }

    pub fn create_list(&self, dto: CreateList) -> Result<List, ApiError> {
        use super::schema::lists;

        diesel::insert_into(lists::table) // insert into lists table
            .values(&dto) // use values from CreateListDTO
            .get_result(&self.connection) // execute query
            .map_err(|err| ApiError::from_diesel_err(err, "while creating list"))
        // if error occurred map it to ApiError
    }

    /// retrieve all lists from the db
    pub fn get_lists(&self) -> Result<Vec<List>, ApiError> {
        use super::schema::lists::dsl::*;

        lists
            .load(&self.connection)
            .map_err(|err| ApiError::from_diesel_err(err, "while listing lists"))
    }

    /// retrieve one list from the db, complete with the the items
    pub fn get_list(&self, list_id: i64) -> Result<(List, Vec<Item>), ApiError> {
        use super::schema::lists::dsl::*;

        match lists.find(list_id).first::<List>(&self.connection) {
            Ok(list) => match Item::belonging_to(&list).load::<Item>(&self.connection) {
                Ok(items) => return Ok((list, items)),
                Err(_) => return Ok((list, vec![])),
            },
            Err(err) => {
                return Err(ApiError::from_diesel_err(
                    err,
                    "while loading single list with items",
                ));
            }
        }
    }

    pub fn update_list(
        &self,
        list_id: i64,
        new_title: String,
        new_subtitle: String,
    ) -> Result<usize, ApiError> {
        use super::schema::lists::dsl::*;

        let updated = diesel::update(lists)
            .filter(id.eq(list_id))
            .set((title.eq(new_title), subtitle.eq(new_subtitle)))
            .execute(&self.connection)
            .map_err(|err| ApiError::from_diesel_err(err, "while updating list"))?;

        if updated == 0 {
            return Err(ApiError::new("List not found", ErrorType::NotFound));
        }
        return Ok(updated);
    }

    pub fn delete_list(&self, list_id: i64) -> Result<usize, ApiError> {
        use super::schema::lists::dsl::*;

        let deleted = diesel::delete(lists.filter(id.eq(list_id)))
            .execute(&self.connection)
            .map_err(|err| ApiError::from_diesel_err(err, "while deleting list"))?;

        if deleted == 0 {
            return Err(ApiError::new("List not found", ErrorType::NotFound));
        }
        return Ok(deleted);
    }

    pub fn create_item(&self, dto: CreateItem) -> Result<Item, ApiError> {
        use super::schema::items;

        diesel::insert_into(items::table) // insert into items table
            .values(&dto) // use values from CreateListDTO
            .get_result(&self.connection) // execute query
            .map_err(|err| ApiError::from_diesel_err(err, "while creating item"))
        // if error occurred map it to ApiError
    }

    pub fn update_item(
        &self,
        item_id: i64,
        new_title: String,
        new_amount: i32,
    ) -> Result<usize, ApiError> {
        use super::schema::items::dsl::*;

        let updated = diesel::update(items)
            .filter(id.eq(item_id))
            .set((title.eq(new_title), amount.eq(new_amount)))
            .execute(&self.connection)
            .map_err(|err| ApiError::from_diesel_err(err, "while updating item"))?;

        if updated == 0 {
            return Err(ApiError::new("Item not found", ErrorType::NotFound));
        }
        return Ok(updated);
    }

    pub fn delete_item(&self, item_id: i64) -> Result<usize, ApiError> {
        use super::schema::items::dsl::*;

        let deleted = diesel::delete(items.filter(id.eq(item_id)))
            .execute(&self.connection)
            .map_err(|err| ApiError::from_diesel_err(err, "while deleting item"))?;

        if deleted == 0 {
            return Err(ApiError::new("Item not found", ErrorType::NotFound));
        }
        return Ok(deleted);
    }
}
