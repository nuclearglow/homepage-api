use crate::errors::ApiError;
use crate::webauthn::actors::*;
use serde::Serialize;
use std::sync::Arc;
use webauthn_rs::proto::{
    CreationChallengeResponse, Credential, CredentialID, PublicKeyCredential,
    RegisterPublicKeyCredential, RequestChallengeResponse, UserId, UserVerificationPolicy,
};

pub async fn challenge_register(
    username: String,
    actor: Arc<WebauthnActor>,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling challenge register");

    let response = actor.challenge_register(username).await;
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
    username: String,
    actor: Arc<WebauthnActor>,
    reg: RegisterPublicKeyCredential,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling register");
    let response = actor.register(&username, &reg).await;
    match response {
        Ok(result) => return respond(Ok(result), warp::http::StatusCode::OK),
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
