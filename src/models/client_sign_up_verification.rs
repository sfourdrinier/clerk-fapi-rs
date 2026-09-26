// src/models/client_sign_up_verification.rs

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientSignUpVerification {
    ClientSdkVerification(Box<models::ClientSdkVerification>),
    StubsSignUpVerification(Box<models::StubsSignUpVerification>),
}
