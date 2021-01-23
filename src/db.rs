use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

use crate::errors::{ApiError, ErrorType};
use crate::models::{CreateListDTO, ListDTO};

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

pub struct DBManager {
    connection: PooledPg,
}

impl DBManager {
    pub fn new(connection: PooledPg) -> DBManager {
        DBManager { connection }
    }

    pub fn create_list(&self, dto: CreateListDTO) -> Result<ListDTO, ApiError> {
        use super::schema::lists;

        diesel::insert_into(lists::table) // insert into lists table
            .values(&dto) // use values from CreateListDTO
            .get_result(&self.connection) // execute query
            .map_err(|err| ApiError::from_diesel_err(err, "while creating list"))
        // if error occurred map it to ApiError
    }

    /// retrieve all lists from the db
    pub fn get_lists(&self) -> Result<Vec<ListDTO>, ApiError> {
        use super::schema::lists::dsl::*;

        lists
            .load(&self.connection)
            .map_err(|err| ApiError::from_diesel_err(err, "while listing lists"))
    }

    pub fn get_list(&self, list_id: i64) -> Result<ListDTO, ApiError> {
        use super::schema::lists::dsl::*;

        match lists
            .filter(id.eq(list_id))
            .load::<ListDTO>(&self.connection)
        {
            Ok(list) => {
                if list.is_empty() {
                    return Err(ApiError::new("No list found", ErrorType::NotFound));
                }
                return Ok(list[0].clone());
            }
            Err(err) => {
                return Err(ApiError::from_diesel_err(err, "while loading single list"));
            }
        }
    }

    pub fn update_list(
        &self,
        list_id: i64,
        new_title: String,
        new_info: String,
    ) -> Result<usize, ApiError> {
        use super::schema::lists::dsl::*;

        let updated = diesel::update(lists)
            .filter(id.eq(list_id))
            .set((title.eq(new_title), info.eq(new_info)))
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
}
