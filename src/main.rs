#[macro_use]
extern crate diesel;

mod api;
mod db;
mod errors;
mod models;
mod routes;
mod schema;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use log::{debug, error, info, trace, warn};
use pretty_env_logger;
use serde::de::DeserializeOwned;
use std::env;
use std::net::SocketAddrV4;
use warp::{reject, Filter};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

fn pg_pool(db_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    return Pool::new(manager).expect("Postgres connection pool could not be created");
}

pub fn with_db_access_manager(
    pool: PgPool,
) -> impl Filter<Extract = (db::DBManager,), Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || pool.clone())
        .and_then(|pool: PgPool| async move {
            match pool.get() {
                Ok(conn) => Ok(db::DBManager::new(conn)),
                Err(err) => Err(warp::reject::custom(errors::ApiError::new(
                    format!("Error getting connection from pool: {}", err.to_string()).as_str(),
                    errors::ErrorType::Internal,
                ))),
            }
        })
}

// TODO: to package db
pub fn with_json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
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
    info!("Log level set to {}", env::var("RUST_LOG").unwrap());

    // get the server address from dotenv
    let server_url: String =
        env::var("API_SERVER_URL").expect("Add required field API_SERVER_URL to your .env file!");
    let server_url: SocketAddrV4 = server_url
        .parse()
        .expect("SERVER_IP field in .env invalid! Use a valid IPv4 Socket Address.");

    // set up database
    let database_url = env::var("DATABASE_URL").expect("Add DATABASE_URL to yur .env file");
    let pg_pool = pg_pool(database_url.as_str());

    // set up the routes
    // Add path prefix /api to all our routes
    let routes = warp::path!("api" / ..)
        .and(
            routes::add_list(pg_pool.clone())
                .or(routes::get_lists(pg_pool.clone()))
                .or(routes::get_list(pg_pool.clone()))
                .or(routes::update_list(pg_pool.clone()))
                .or(routes::delete_list(pg_pool)),
        )
        .recover(errors::handle_rejection);

    info!("Warp starting on http://{:?}", server_url);

    // serve async
    warp::serve(routes).run(server_url).await;
}
