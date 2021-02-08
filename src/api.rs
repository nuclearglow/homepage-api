use crate::db;
use crate::errors::ApiError;
use crate::models::{CreateItem, CreateList, Item, List};
use serde::Serialize;
use serde_derive::{Deserialize, Serialize};

// Api List Wrapper Struct
#[derive(Debug, Deserialize, Clone)]
pub struct AddList {
    pub title: String,
    pub subtitle: String,
}

impl AddList {
    pub fn to_dto(&self) -> CreateList {
        CreateList {
            title: self.title.clone(),
            subtitle: self.subtitle.clone(),
        }
    }
}

// Api Item Wrapper Struct
#[derive(Debug, Deserialize, Clone)]
pub struct AddItem {
    pub list_id: i64,
    pub title: String,
    pub amount: i32,
}

impl AddItem {
    pub fn to_dto(&self) -> CreateItem {
        CreateItem {
            list_id: self.list_id.clone(),
            title: self.title.clone(),
            amount: self.amount.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateItem {
    pub title: String,
    pub amount: i32,
}

#[derive(Debug, Serialize, Clone)]
pub struct ListWithItems {
    pub id: i64,
    pub title: String,
    pub subtitle: String,
    pub items: Vec<Item>,
}

impl ListWithItems {
    pub fn new(list: List, items: Vec<Item>) -> ListWithItems {
        ListWithItems {
            id: list.id,
            title: list.title,
            subtitle: list.subtitle,
            items,
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
        .map(|list| IdResponse::new(list.id));

    return respond(id_response, warp::http::StatusCode::CREATED);
}

pub async fn get_lists(db_manager: db::DBManager) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling get lists");

    let result = db_manager.get_lists();

    return respond(result, warp::http::StatusCode::OK);
}

pub async fn get_list(
    list_id: i64,
    db_manager: db::DBManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling get single list");

    // retrieve list and associated items from db
    let result = db_manager.get_list(list_id);
    match result {
        // list is found, return data and 200
        Ok((list, items)) => {
            // return as list with items
            let result = Ok(ListWithItems::new(list, items));
            return respond(result, warp::http::StatusCode::OK);
        }
        // list was not found, return 404
        Err(err) => return respond(Err(err), warp::http::StatusCode::NOT_FOUND),
    };
}

pub async fn update_list(
    list_id: i64,
    db_manager: db::DBManager,
    updated_list: AddList,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling update status");

    let id_response = db_manager
        .update_list(list_id, updated_list.title, updated_list.subtitle)
        .map(|_| IdResponse::new(list_id));

    return respond(id_response, warp::http::StatusCode::OK);
}

pub async fn delete_list(
    list_id: i64,
    db_manager: db::DBManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling delete list");

    let result = db_manager.delete_list(list_id).map(|_| -> () { () });

    return respond(result, warp::http::StatusCode::NO_CONTENT);
}

pub async fn add_item(
    db_manager: db::DBManager,
    new_item: AddItem,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling add item");

    let create_item = new_item.to_dto();

    let id_response = db_manager
        .create_item(create_item)
        .map(|list| IdResponse::new(list.id));

    return respond(id_response, warp::http::StatusCode::CREATED);
}

pub async fn update_item(
    item_id: i64,
    db_manager: db::DBManager,
    updated_item: UpdateItem,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("updating item {}", item_id);

    let id_response = db_manager
        .update_item(item_id, updated_item.title, updated_item.amount)
        .map(|_| IdResponse::new(item_id));

    return respond(id_response, warp::http::StatusCode::OK);
}

pub async fn delete_item(
    item_id: i64,
    db_manager: db::DBManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("deleting item {}", item_id);

    let result = db_manager.delete_item(item_id).map(|_| -> () { () });

    return respond(result, warp::http::StatusCode::NO_CONTENT);
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
