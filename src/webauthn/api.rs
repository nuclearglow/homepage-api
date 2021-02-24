use serde::Serialize;
use std::sync::Arc;
use webauthn_rs::proto::{
    CreationChallengeResponse, Credential, CredentialID, PublicKeyCredential,
    RegisterPublicKeyCredential, RequestChallengeResponse, UserId, UserVerificationPolicy,
};

use crate::db;
use crate::errors::ApiError;
use crate::webauthn::actors::*;
use crate::webauthn::routes::RegisterData;

pub async fn challenge_register(
    nick: String,
    actor: Arc<WebauthnActor>,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling challenge register");

    let response = actor.challenge_register(nick).await;
    match response {
        Ok(challenge) => return respond(Ok(challenge), warp::http::StatusCode::OK),
        Err(err) => {
            return respond(
                Err(ApiError::from_webauthn_error(err, "challenge register")),
                warp::http::StatusCode::UNAUTHORIZED,
            )
        }
    }
}

pub async fn register(
    register_data: RegisterData,
    actor: Arc<WebauthnActor>,
    db_manager: db::DBManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling register");
    let response = actor
        .register(register_data.user, register_data.credentials, db_manager)
        .await;
    match response {
        Ok(user_id) => return respond(Ok(user_id), warp::http::StatusCode::OK),
        Err(err) => {
            return respond(
                Err(ApiError::from_webauthn_error(err, "register")),
                warp::http::StatusCode::UNAUTHORIZED,
            )
        }
    }
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
            log::error!("Error while responding in Webauthn: {}", err.to_string());
            Err(warp::reject::custom(err))
        }
    }
}
