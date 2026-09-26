// src/models/client_sdk_verification.rs

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientSdkVerification {
    pub object: Object,
    pub id: String,
    pub status: Option<Status>,
    pub strategy: Option<String>,
    pub nonce: Option<String>,
    pub message: Option<String>,
    pub external_verification_redirect_url: Option<String>,
    pub attempts: Option<i64>,
    pub expire_at: Option<i64>,
    pub error: Option<Box<ClientSdkVerificationError>>,
    pub verified_at_client: Option<String>,
    pub next_action: Option<String>,
    pub supported_strategies: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientSdkVerificationError {
    pub code: String,
    pub message: String,
    pub long_message: Option<String>,
    pub meta: Option<ClientSdkVerificationErrorMeta>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientSdkVerificationErrorMeta {
    pub param_name: Option<String>,
    pub session_id: Option<String>,
    pub email_addresses: Option<Vec<String>>,
    pub identifiers: Option<Vec<String>>,
    pub zxcvbn: Option<ClientSdkVerificationZxcvbn>,
    pub plan: Option<ClientSdkVerificationPlan>,
    pub is_plan_upgrade_possible: Option<bool>,
    pub seats_quantity_to_add: Option<i64>,
    pub seats_quantity: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientSdkVerificationZxcvbn {
    pub suggestions: Vec<String>,
    pub warning: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientSdkVerificationPlan {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "verification")]
    Verification,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "unverified")]
    Unverified,
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "transferable")]
    Transferable,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "expired")]
    Expired,
}
