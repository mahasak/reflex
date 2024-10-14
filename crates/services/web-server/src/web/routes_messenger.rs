use axum::extract::{FromRef, Query, State};
use axum::{debug_handler, Json, Router};
use axum::routing::{get, post};
use serde_json::{json, Value};
use tower_cookies::Cookies;
use tracing::{debug, info};
use lib_core::cache::CacheService;
use lib_core::model::ModelManager;
use crate::web_config;
use lib_core::ctx::Ctx;
use lib_core::model::merchant_channel::{MerchantChannel, MerchantChannelBmc};
use lib_core::model::merchant_config::{MerchantConfig, MerchantConfigBmc};
use lib_core::model::messenger_webhook::{MessengerVerifySubscription, MessengerWebhook};
use crate::web::mw_auth::CtxW;

use crate::web::{self, remove_token_cookie, Error, Result};
#[derive(Clone, FromRef)]
pub struct SharedState {
    pub(crate) mm: ModelManager,
    pub(crate) cache: CacheService,
}

pub fn routes(mm: ModelManager, cache: CacheService) -> Router {
    Router::new()
        .route("/v1/messenger", post(messenger_post_handler))
        .route("/v1/messenger", get(messenger_get_handler))
        .with_state(SharedState{ mm, cache})
}

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

    if hub_mode == "subscribe" && verify_token == web_config().VERIFY_TOKEN {
        hub_challenge.to_string()
    } else {
        "Verification failed".to_string()
    }
}

async fn messenger_post_handler(
    State(_mm): State<ModelManager>,
    State(_cache): State<CacheService>,
    Json(payload): Json<MessengerWebhook>,
) -> Result<Json<Value>> {
    debug!("{:<12} - messenger_post_handler", "HANDLER");
    let ctx = Ctx::root_ctx();
    let work_payload = payload.clone();
    let object = work_payload.object;

    if object == "page" {

        for entry in work_payload.entry.iter() {
            let page_id = entry.id.clone();
            let merchant_channel = MerchantChannelBmc::get_by_ref_id::<MerchantChannel>(
                &ctx,
                &_mm,
                &page_id.clone()
            ).await?;

            match merchant_channel {
                None => info!("{:<12} - Eligibility: No, Page ID {} is not registered", "MESSENGER_WEBHOOK", page_id.clone()),
                Some(merchant_channel) => {
                    let app_config = MerchantConfigBmc::get_by_channel_id::<MerchantConfig>(
                            &ctx,
                            &_mm,
                            &merchant_channel.id
                        ).await?;
                    match app_config {
                        None => info!("{:<12} - Eligibility: No, Page ID {} is not assigned", "MESSENGER_WEBHOOK", page_id.clone()),
                        Some(app_config) => {
                            info!("{:<12} - Eligibility: Yes, Page ID {} is registered and assigned", "MESSENGER_WEBHOOK", page_id.clone());
                            let json_payload = payload.clone();
                            let json_str = serde_json::to_string(&json_payload)?;
                        }
                    }
                }
            }
        }
    } else {
        info!("{:<12} - Received non-page object, Got {:?}", "MESSENGER_WEBHOOK", object);
    }

    let body = Json(json!({
		"result": {
			"success": true
		}
	}));

    Ok(body)
}