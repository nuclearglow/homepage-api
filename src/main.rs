#[macro_use]
extern crate diesel;

mod api;
mod db;
mod errors;
mod models;
mod routes;
mod schema;
mod webauthn;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use log::info;
use log::LevelFilter;
use pretty_env_logger;
use serde::de::DeserializeOwned;
use std::env;
use std::net::SocketAddrV4;
use std::sync::Arc;
use warp::Filter;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
use webauthn_rs::ephemeral::WebauthnEphemeralConfig;

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
    // init .env
    dotenv().ok();

    // init logging
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::formatted_timed_builder()
        // https://docs.rs/env_logger/0.5.0-rc.1/env_logger/struct.Builder.html
        // .target(Target::Stdout)
        .filter(None, LevelFilter::Info)
        .init();

    // pretty_env_logger::init();
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

    // set up webauthn
    let wan_c =
        WebauthnEphemeralConfig::new("localhost", "http://localhost:8888", "localhost", None);

    let wan = crate::webauthn::actors::WebauthnActor::new(wan_c);
    let actor = Arc::new(wan);

    // set up the routes
    // Add path prefix /api to all our routes
    let routes = warp::path!("api" / ..)
        .and(
            // webauthn routes
            crate::webauthn::routes::challenge_register(actor)
                // list routes
                .or(routes::add_list(pg_pool.clone()))
                .or(routes::get_lists(pg_pool.clone()))
                .or(routes::get_list(pg_pool.clone()))
                .or(routes::update_list(pg_pool.clone()))
                .or(routes::delete_list(pg_pool.clone()))
                // item routes
                .or(routes::add_item(pg_pool.clone()))
                .or(routes::update_item(pg_pool.clone()))
                .or(routes::delete_item(pg_pool)),
        )
        .recover(errors::handle_rejection);

    info!("Warp starting on http://{:?}", server_url);

    // serve async
    warp::serve(routes).run(server_url).await;
}
