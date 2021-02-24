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
use log::LevelFilter;
use log::*;
use pretty_env_logger;
use serde::de::DeserializeOwned;
use std::env;
use std::net::SocketAddrV4;
use std::str::FromStr;
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

    let log_level: String = env::var("RUST_LOG").unwrap();
    let level_filter: LevelFilter = match LevelFilter::from_str(&log_level) {
        Ok(level) => level,
        Err(_) => LevelFilter::Debug,
    };

    pretty_env_logger::formatted_timed_builder()
        // https://docs.rs/env_logger/0.5.0-rc.1/env_logger/struct.Builder.html
        // TODO: .target(Target::Stdout) TODO: in prod mode, should log to file
        .filter(None, level_filter)
        .init();

    info!("Log level set to {}", level_filter);

    debug!("Using Environment");
    for (key, value) in env::vars() {
        debug!("{}: {}", key, value);
    }

    // get the server address from dotenv
    let server_url: String =
        env::var("API_SERVER_URL").expect("Add required field API_SERVER_URL to your .env file!");
    let server_url: SocketAddrV4 = server_url
        .parse()
        .expect("SERVER_IP field in .env invalid! Use a valid IPv4 Socket Address.");

    // set up database
    let database_url = env::var("DATABASE_URL").expect("Add DATABASE_URL to yur .env file");
    let pg_pool = pg_pool(database_url.as_str());

    // set up Webauthn relying party parameters
    let webauthn_rp_name = env::var("WEBAUTHN_RELYING_PARTY_NAME")
        .expect("Add WEBAUTHN_RELYING_PARTY_NAME to yur .env file");
    info!("Webauthn Relying Party Name {:?} ", webauthn_rp_name);
    let webauthn_rp_origin = env::var("WEBAUTHN_RELYING_PARTY_ORIGIN")
        .expect("Add WEBAUTHN_RELYING_PARTY_ORIGIN to yur .env file");
    info!("Webauthn Relying Party Origin {:?} ", webauthn_rp_origin);
    let webauthn_rp_id = env::var("WEBAUTHN_RELYING_PARTY_ID")
        .expect("Add WEBAUTHN_RELYING_PARTY_ID to yur .env file");
    info!("Webauthn Relying Party Id {:?} ", webauthn_rp_id);

    // set up Webauthn Ephemeral Config
    let wan_c = WebauthnEphemeralConfig::new(
        webauthn_rp_name.as_str(),
        webauthn_rp_origin.as_str(),
        webauthn_rp_id.as_str(),
        None,
    );

    // create Actor
    let wan = crate::webauthn::actors::WebauthnActor::new(wan_c);
    let actor = Arc::new(wan);

    // set up the routes

    // Webauthn: Add path prefix /auth to all these routes
    let auth_routes = warp::path!("auth" / ..).and(
        webauthn::routes::challenge_register(actor.clone())
            .or(webauthn::routes::register(pg_pool.clone(), actor.clone()))
            .or(webauthn::routes::challenge_login(actor)),
    );

    // API: Add path prefix /api to all our routes
    let api_routes = warp::path!("api" / ..).and(
        // list routes
        routes::add_list(pg_pool.clone())
            .or(routes::get_lists(pg_pool.clone()))
            .or(routes::get_list(pg_pool.clone()))
            .or(routes::update_list(pg_pool.clone()))
            .or(routes::delete_list(pg_pool.clone()))
            // item routes
            .or(routes::add_item(pg_pool.clone()))
            .or(routes::update_item(pg_pool.clone()))
            .or(routes::delete_item(pg_pool)),
    );

    // assemble all routes, add error handler
    let routes = auth_routes.or(api_routes).recover(errors::handle_rejection);

    info!("Warp starting on http://{:?}", server_url);

    // serve async
    warp::serve(routes).run(server_url).await;
}
