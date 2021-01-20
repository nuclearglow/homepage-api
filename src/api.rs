use crate::db;
use crate::errors::ApiError;
use crate::models::CreateListDTO;
use serde::Serialize;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct AddList {
    pub title: String,
    pub info: String,
}

impl AddList {
    pub fn to_dto(&self) -> CreateListDTO {
        CreateListDTO {
            title: self.title.clone(),
            info: self.info.clone(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct IdResponse {
    pub id: i64,
}
impl IdResponse {
    pub fn new(id: i64) -> IdResponse {
        IdResponse { id }
    }
}

pub async fn add_list(
    db_manager: db::DBManager,
    new_list: AddList,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling add list");

    let create_list_dto = new_list.to_dto();

    let id_response = db_manager
        .create_list(create_list_dto)
        .map(|book| IdResponse::new(book.id));

    respond(id_response, warp::http::StatusCode::CREATED)
}

pub async fn list_lists(db_manager: db::DBManager) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling list books");

    let result = db_manager.list_lists();

    respond(result, warp::http::StatusCode::OK)
}

pub async fn update_list(
    list_id: i64,
    db_manager: db::DBManager,
    updated_list: AddList,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling update status");

    let id_response = db_manager
        .update_list(list_id, updated_list.title, updated_list.info)
        .map(|_| IdResponse::new(list_id));

    respond(id_response, warp::http::StatusCode::OK)
}

pub async fn delete_list(
    list_id: i64,
    db_manager: db::DBManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling delete book");

    let result = db_manager.delete_list(list_id).map(|_| -> () { () });

    respond(result, warp::http::StatusCode::NO_CONTENT)
}

fn respond<T: Serialize>(
    result: Result<T, ApiError>,
    status: warp::http::StatusCode,
) -> Result<impl warp::Reply, warp::Rejection> {
    match result {
        Ok(response) => Ok(warp::reply::with_status(
            warp::reply::json(&response),
            status,
        )),
        Err(err) => {
            log::error!("Error while trying to respond: {}", err.to_string());
            Err(warp::reject::custom(err))
        }
    }
}
