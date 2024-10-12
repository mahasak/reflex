use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessengerVerifySubscription {
    #[serde(alias = "hub.mode")]
    pub hub_mode: Option<String>,
    #[serde(alias = "hub.verify_token")]
    pub hub_verify_token: Option<String>,
    #[serde(alias = "hub.challenge")]
    pub hub_challenge: Option<String>,
}
