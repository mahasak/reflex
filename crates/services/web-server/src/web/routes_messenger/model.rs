use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessengerVerifySubscription {
    #[serde(alias = "hub.mode")]
    pub hub_mode: Option<String>,
    #[serde(alias = "hub.verify_token")]
    pub hub_verify_token: Option<String>,
    #[serde(alias = "hub.challenge")]
    pub hub_challenge: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct Sender {
    pub id: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct Recipient {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct QuickReplyPayload {
    pub payload: String,
}

#[allow(dead_code)]
impl QuickReplyPayload {
    pub fn get_payload(&self) -> &String {
        &self.payload
    }
}

#[allow(dead_code)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct Message {
    pub text: Option<String>,
    pub quick_reply: Option<QuickReplyPayload>,
}

#[allow(dead_code)]
impl Message {
    pub fn get_text(&self) -> String {
        self.text.clone().unwrap_or_default()
    }

    pub fn get_quick_reply(&self) -> Option<QuickReplyPayload> {
        self.quick_reply.clone()
    }
}

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct Postback {
    pub payload: String,
}

#[allow(dead_code)]
impl MessagePostback {
    pub fn get_payload(&self) -> &String {
        &self.payload
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryInfo {
    pub mids: Vec<String>,
    pub watermark: Number,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadInfo {
    pub watermark: Number,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountLinkingInfo {
    pub status: String,
    pub authorization_code: String,
}

#[allow(dead_code)]
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Messaging {
    pub sender: Sender,
    pub postback: Option<MessagePostback>,
    pub message: Option<Message>,
    pub delivery: Option<DeliveryInfo>,
    pub read: Option<ReadInfo>,
    pub account_linking: Option<AccountLinkingInfo>,
    pub recipient: Recipient,
    pub reaction: Option<String>,
    pub timestamp: Number,
}

#[allow(dead_code)]
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebhookEntry {
    pub id: String,
    pub time: Number,
    pub messaging: Option<Vec<Messaging>>,
    pub changes: Option<Vec<ChangesEvent>>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChangesEvent {
    pub field: String,
    pub value: Option<ChangeEventValue>,
}

#[allow(dead_code)]
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChangeEventValue {
    pub page_id: String,            //Generic field
    pub invoice_id: Option<String>, // P2M invoice field
    pub media_id: Option<String>,   // P2M Bankslip field
    pub buyer_id: Option<String>,   // P2M Bankslip field
    pub timestamp: Number,
    pub event: Option<String>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WrappedMessage {
    pub trace_id: String,
    pub page_entry: WebhookEntry,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessengerWebhook {
    pub object: String,
    pub entry: Vec<WebhookEntry>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessagePostback {
    pub payload: String,
}

#[allow(dead_code)]
impl Postback {
    pub fn get_payload(&self) -> &String {
        &self.payload
    }
}