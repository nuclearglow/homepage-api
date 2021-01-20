use crate::api;
use crate::with_db_access_manager;
use crate::with_json_body;
use crate::PgPool;

use warp::Filter;

/// GET /lists
pub fn list_lists(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("lists")
        .and(warp::get())
        .and(with_db_access_manager(pool))
        .and_then(api::list_lists)
}

/// POST /lists
pub fn add_list(
    pool: PgPool,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("lists") // Match /books path
        .and(warp::post()) // Match POST method
        .and(with_db_access_manager(pool)) // Add DBAccessManager to params tuple
        .and(with_json_body::<api::AddList>()) // Try to deserialize JSON body to AddBook
        .and_then(api::add_list) // Pass the params touple to the handler function
}

/// PUT /lists/:id
pub fn update_list(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("lists" / i64)
        .and(warp::put())
        .and(with_db_access_manager(pool))
        .and(with_json_body::<api::AddList>()) // Try to deserialize JSON body to AddBook
        .and_then(api::update_list)
}

/// DELETE /lists/:id
pub fn delete_list(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("lists" / i64)
        .and(warp::delete())
        .and(with_db_access_manager(pool))
        .and_then(api::delete_list)
}
