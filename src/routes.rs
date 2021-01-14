use std::collections::HashMap;
use warp::{http, Filter};

use crate::localstore;

pub async fn add_item(
    item: localstore::Item,
    store: localstore::Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    store.list.write().insert(item.name, item.quantity);

    Ok(warp::reply::with_status(
        "Added items to the list",
        http::StatusCode::CREATED,
    ))
}

pub async fn get_items(store: localstore::Store) -> Result<impl warp::Reply, warp::Rejection> {
    let mut result = HashMap::new();
    let r = store.list.read();
    for (key, valiue) in r.iter() {
        result.insert(key, valiue);
    }

    return Ok(warp::reply::json(&result));
}

pub async fn delete_item(
    item: localstore::Item,
    store: localstore::Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    store.list.write().remove(&item.name);

    return Ok(warp::reply::with_status(
        format!("Removed item {}", item.name),
        http::StatusCode::OK,
    ));
}
