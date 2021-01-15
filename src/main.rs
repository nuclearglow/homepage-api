use dotenv::dotenv;
use log::{debug, error, info, trace, warn};
use pretty_env_logger;
use std::env;
use std::net::SocketAddrV4;
use warp::Filter;

mod localstore;
mod routes;

fn json_body() -> impl Filter<Extract = (localstore::Item,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // and to reject huge payloads we set a content length limit
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[tokio::main]
async fn main() {
    // init dotenv
    dotenv().ok();

    // init logging
    pretty_env_logger::init();
    let log = warp::log("info");

    // get the server address from dotenv
    let server_url: String =
        env::var("API_SERVER_URL").expect("Add required field API_SERVER_URL to your .env file!");
    let server_url: SocketAddrV4 = server_url
        .parse()
        .expect("SERVER_IP field in .env invalid! Use a valid IPv4 Socket Address.");
    info!("Warp starting on http://{:?}", server_url);

    // create a new store
    let store = localstore::Store::new();
    //  we need to pass the store down to each method by cloning it and creating a warp filter, which we call in the .and() during the warp path creation.
    let store_filter = warp::any().map(move || store.clone());

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
    let routes = ping.or(add_item).or(get_items).or(delete_item).with(log);

    // serve async
    warp::serve(routes).run(server_url).await;
}
