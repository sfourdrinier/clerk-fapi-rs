// src/models/client_passkey_verification.rs

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientPasskeyVerification {
    ClientSdkVerification(Box<models::ClientSdkVerification>),
    StubsVerificationPasskey(Box<models::StubsVerificationPasskey>),
}
