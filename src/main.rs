#[macro_use]
extern crate diesel;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use log::{debug, error, info, trace, warn};
use pretty_env_logger;
use std::env;
use std::net::SocketAddrV4;
use warp::Filter;

mod data_access;
mod errors;
mod models;
mod routes;
mod schema;

type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

fn mysql_pool(db_url: &str) -> MysqlPool {
    let manager = ConnectionManager::<MysqlConnection>::new(db_url);
    Pool::new(manager).expect("MySQL connection pool could not be created")
}

use crate::data_access::DBAccessManager;
use crate::errors::{ApiError, ErrorType};

fn with_db_access_manager(
    pool: MysqlPool,
) -> impl Filter<Extract = (DBAccessManager,), Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || pool.clone())
        .and_then(|pool: MysqlPool| async move {
            match pool.get() {
                Ok(conn) => Ok(DBAccessManager::new(conn)),
                Err(err) => Err(warp::reject::custom(ApiError::new(
                    format!("Error getting connection from pool: {}", err.to_string()).as_str(),
                    ErrorType::Internal,
                ))),
            }
        })
}

#[tokio::main]
async fn main() {
    // init dotenv
    dotenv().ok();

    // init logging
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    let log = warp::log("info");

    // get the server address from dotenv
    let server_url: String =
        env::var("API_SERVER_URL").expect("Add required field API_SERVER_URL to your .env file!");
    let server_url: SocketAddrV4 = server_url
        .parse()
        .expect("SERVER_IP field in .env invalid! Use a valid IPv4 Socket Address.");
    info!("Warp starting on http://{:?}", server_url);

    // set up database
    let database_url = env::var("DATABASE_URL").expect("Add DATABASE_URL to yur .env file");
    let mysql_pool = mysql_pool(database_url.as_str());

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let ping = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    // define URL paths and handler functions
    // let add_item = warp::post()
    //     .and(warp::path("list"))
    //     .and(warp::path::end())
    //     .and(json_body())
    //     .and(store_filter.clone())
    //     .and_then(routes::add_item);

    // let get_items = warp::get()
    //     .and(warp::path("list"))
    //     .and(warp::path::end())
    //     .and(store_filter.clone())
    //     .and_then(routes::get_items);

    // let delete_item = warp::delete()
    //     .and(warp::path("list"))
    //     .and(warp::path::end())
    //     .and(json_body())
    //     .and(store_filter.clone())
    //     .and_then(routes::delete_item);

    // assemble active routes
    let routes = ping;
    // .or(add_item).or(get_items).or(delete_item).with(log);

    // serve async
    warp::serve(routes).run(server_url).await;
}
