use crate::errors::ApiError;
use crate::webauthn::actors::*;
use serde::Serialize;
use std::sync::Arc;

pub async fn challenge_register(
    username: String,
    actor: Arc<WebauthnActor>,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling challenge register");

    let response = actor.challenge_register(username).await;
    match response {
        Ok(challenge) => return respond(Ok(challenge), warp::http::StatusCode::OK),
        Err(err) =>
        // TODO: from_webauthn_error just like with diesel
        {
            log::error!("Webauthn Error: {:?}", err);
            return respond(
                Err(ApiError::new(
                    "Webauthn Error",
                    crate::errors::ErrorType::BadRequest,
                )),
                warp::http::StatusCode::UNAUTHORIZED,
            );
        }
    };
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
            log::error!("Error while trying to respond: {}", err.to_string());
            Err(warp::reject::custom(err))
        }
    }
}
