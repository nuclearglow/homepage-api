use crate::api;
use crate::with_db_access_manager;
use crate::with_json_body;
use crate::PgPool;

use warp::Filter;

/// Admin: Get all lists
/// GET /lists
pub fn get_lists(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("lists")
        .and(warp::get())
        .and(with_db_access_manager(pool))
        .and_then(api::get_lists)
}

/// GET /list/:id
pub fn get_list(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("list" / i64)
        .and(warp::get())
        .and(with_db_access_manager(pool))
        .and_then(api::get_list)
}

/// POST /list
pub fn add_list(
    pool: PgPool,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("list") // Match /lists path
        .and(warp::post()) // Match POST method
        .and(with_db_access_manager(pool)) // Add DBAccessManager to params tuple
        .and(with_json_body::<api::AddList>()) // Try to deserialize JSON body to AddList
        .and_then(api::add_list) // Pass the params touple to the handler function
}

/// PUT /list/:id/
pub fn update_list(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("list" / i64)
        .and(warp::put())
        .and(with_db_access_manager(pool))
        .and(with_json_body::<api::AddList>()) // Try to deserialize JSON body to AddList
        .and_then(api::update_list)
}

/// DELETE /list/:id
pub fn delete_list(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("list" / i64)
        .and(warp::delete())
        .and(with_db_access_manager(pool))
        .and_then(api::delete_list)
}

/// POST /item
pub fn add_item(
    pool: PgPool,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("item") // Match /item path
        .and(warp::post()) // Match POST method
        .and(with_db_access_manager(pool)) // Add DBManager to params tuple
        .and(with_json_body::<api::AddItem>()) // Try to deserialize JSON body to AddList
        .and_then(api::add_item) // Pass the params touple to the handler function
}

/// PUT /item/:id/
pub fn update_item(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("item" / i64)
        .and(warp::put())
        .and(with_db_access_manager(pool))
        .and(with_json_body::<api::UpdateItem>()) // Try to deserialize JSON body to AddList
        .and_then(api::update_item)
}

/// DELETE /item/:id
pub fn delete_item(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("item" / i64)
        .and(warp::delete())
        .and(with_db_access_manager(pool))
        .and_then(api::delete_item)
}
