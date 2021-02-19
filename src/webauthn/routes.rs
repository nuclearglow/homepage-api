use crate::webauthn::actors::*;
use std::sync::Arc;
use warp::Filter;

pub fn with_webauthn_actor(
    actor: Arc<WebauthnActor>,
) -> impl Filter<Extract = (Arc<WebauthnActor>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || actor.clone())
}

/// POST auth/challenge/register/<username>
pub fn challenge_register(
    actor: Arc<WebauthnActor>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("auth" / "challenge" / "register" / String) // Match username
        .and(warp::post()) // Match POST method
        .and(with_webauthn_actor(actor)) // Add the actor
        .and_then(crate::webauthn::api::challenge_register) // Use api method to handle it
}
