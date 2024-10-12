use axum::extract::{Query, State};
use axum::{debug_handler, Router};
use axum::routing::{get};
use tower_cookies::Cookies;
use tracing::debug;
use lib_core::model::ModelManager;
use config::messenger_config;
use crate::web::routes_messenger::model::MessengerVerifySubscription;

mod config;
mod error;
mod model;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/v1/messenger", get(messenger_get_handler))
        .with_state(mm)
}

#[debug_handler]
async fn messenger_get_handler(
    State(_mm): State<ModelManager>,
    _cookies: Cookies,
    Query(query): Query<MessengerVerifySubscription>,
)  -> String {
    debug!("{:<12} - messenger_get_handler", "HANDLER");

    let verify_token = match query.hub_verify_token {
        Some(token) => token,
        None => {
            return "No verify token".to_string();
        }
    };

    let hub_mode = match query.hub_mode {
        Some(mode) => mode,
        None => {
            return "No hub mode".to_string();
        }
    };

    let hub_challenge = match query.hub_challenge {
        Some(challenge) => challenge,
        None => {
            return "No hub challenge".to_string();
        }
    };

    if hub_mode == "subscribe" && verify_token == messenger_config().VERIFY_TOKEN {
        hub_challenge.to_string()
    } else {
        "Verification failed".to_string()
    }
}