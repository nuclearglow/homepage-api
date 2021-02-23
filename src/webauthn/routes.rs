use std::sync::Arc;
use warp::Filter;
use webauthn_rs::proto::RegisterPublicKeyCredential;

use crate::webauthn;
use crate::webauthn::actors::*;
use crate::with_json_body;

pub fn with_webauthn_actor(
    actor: Arc<WebauthnActor>,
) -> impl Filter<Extract = (Arc<WebauthnActor>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || actor.clone())
}

/// POST auth/challenge/register/<username>
pub fn challenge_register(
    actor: Arc<WebauthnActor>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("challenge" / "register" / String) // Match username
        .and(warp::post()) // Match POST method
        .and(with_webauthn_actor(actor)) // Add the actor
        .and_then(webauthn::api::challenge_register) // Use api method to handle it
}

/// POST /auth/register/<username>
pub fn register(
    actor: Arc<WebauthnActor>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("register" / String) // Match username
        .and(warp::post()) // Match POST method
        .and(with_webauthn_actor(actor)) // Add the actor
        .and(with_json_body::<RegisterPublicKeyCredential>()) // Try to deserialize JSON body to RegisterPublicKeyCredential
        .and_then(webauthn::api::register) // Use api method to handle it
}
