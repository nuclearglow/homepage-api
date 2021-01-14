use std::net::IpAddr;
use warp::Filter;

mod localstore;
mod routes;

fn json_body() -> impl Filter<Extract = (localstore::Item,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

// inspired by https://blog.logrocket.com/creating-a-rest-api-in-rust-with-warp/

#[tokio::main]
async fn main() {
    let ip: IpAddr = "0.0.0.0".parse().unwrap();
    let port: u16 = 9090;

    // create a new store
    let store = localstore::Store::new();
    //  we need to pass the store down to each method by cloning it and creating a warp filter, which we call in the .and() during the warp path creation.
    let store_filter = warp::any().map(move || store.clone());

    println!("Starting Warp on http://{:?}:{}", ip, port);

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let ping = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    // define URL paths and handler functions

    let add_item = warp::post()
        .and(warp::path("list"))
        .and(warp::path::end())
        .and(json_body())
        .and(store_filter.clone())
        .and_then(routes::add_item);

    let get_items = warp::get()
        .and(warp::path("list"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::get_items);

    let delete_item = warp::delete()
        .and(warp::path("list"))
        .and(warp::path::end())
        .and(json_body())
        .and(store_filter.clone())
        .and_then(routes::delete_item);

    // assemble active routes
    let routes = ping.or(add_item).or(get_items).or(delete_item);

    warp::serve(routes).run((ip, port)).await;
}
