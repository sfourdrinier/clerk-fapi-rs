// tests/client_sdk_snapshot.rs

use clerk_fapi_rs::models::{
    ClientClient, ClientEmailAddressVerification, ClientPasskeyVerification, ClientSdkVerification,
    ClientSignInFirstFactorVerification, ClientSignUpVerification,
    ExternalAccountWithVerificationVerification,
};
use serde_json::{json, Value};

fn sdk_verification(strategy: Option<&str>, error: Value) -> Value {
    let error = if error.is_null() {
        json!({"code": "", "message": "", "meta": {}})
    } else {
        error
    };
    json!({
        "object": "verification",
        "id": "",
        "status": null,
        "strategy": strategy,
        "nonce": null,
        "message": null,
        "external_verification_redirect_url": null,
        "attempts": null,
        "expire_at": null,
        "error": error
    })
}

#[test]
fn client_sdk_snapshot_deserializes_generic_verifications_without_losing_session_data() {
    let error = json!({
        "code": "form_identifier_not_found",
        "message": "Identifier was not found",
        "meta": {}
    });
    let generic = sdk_verification(None, error);
    let mut google = sdk_verification(Some("oauth_google"), Value::Null);
    google["status"] = json!("verified");
    google["verified_at_client"] = json!("client_snapshot");
    let apple = sdk_verification(Some("oauth_apple"), Value::Null);
    let mut user = json!({
        "object": "user", "id": "user_snapshot", "external_id": null,
        "first_name": null, "last_name": null, "username": null,
        "public_metadata": {}, "unsafe_metadata": {}, "image_url": "", "has_image": false,
        "email_addresses": [], "phone_numbers": [], "web3_wallets": [], "external_accounts": [],
        "passkeys": [], "organization_memberships": [], "enterprise_accounts": [],
        "totp_enabled": false, "backup_code_enabled": false, "two_factor_enabled": false,
        "create_organization_enabled": false, "create_organizations_limit": 0,
        "delete_self_enabled": false, "primary_email_address_id": null,
        "primary_phone_number_id": null, "primary_web3_wallet_id": null,
        "password_enabled": false, "profile_image_id": "", "last_sign_in_at": null,
        "legal_accepted_at": null, "updated_at": null, "created_at": null
    });
    user["email_addresses"] = json!([{
        "id": "idn_email_snapshot", "object": "email_address", "email_address": "snapshot@example.test",
        "verification": generic,
        "linked_to": [{"object": "identification_link", "type": "oauth_google", "id": "idn_google_snapshot"}],
        "matches_sso_connection": false
    }]);
    user["phone_numbers"] = json!([{
        "id": "idn_phone_snapshot", "object": "phone_number", "phone_number": "+15555550100",
        "reserved_for_second_factor": false, "default_second_factor": false,
        "verification": google.clone(), "linked_to": [], "backup_codes": null
    }]);
    user["web3_wallets"] = json!([{
        "id": "idn_wallet_snapshot", "object": "web3_wallet", "web3_wallet": "0x0000000000000000000000000000000000000001",
        "verification": apple
    }]);
    user["passkeys"] = json!([{
        "id": "pk_snapshot", "object": "passkey", "name": null,
        "verification": sdk_verification(Some("passkey"), Value::Null), "last_used_at": null,
        "created_at": 1731327798987i64, "updated_at": 1731327903492i64
    }]);
    user["external_accounts"] = json!([
        {
            "object": "external_account", "id": "idn_google_snapshot", "provider": "google",
            "identification_id": "google-subject", "provider_user_id": "google-subject", "approved_scopes": "email",
            "email_address": "snapshot@example.test", "first_name": "Snapshot", "last_name": "User",
            "avatar_url": null, "image_url": null, "username": null, "phone_number": null,
            "public_metadata": {}, "label": null,
            "verification": google
        },
        {
            "object": "external_account", "id": "idn_apple_snapshot", "provider": "apple",
            "identification_id": "apple-subject", "provider_user_id": "apple-subject", "approved_scopes": "email",
            "email_address": "snapshot@example.test", "first_name": "Snapshot", "last_name": "User",
            "avatar_url": null, "image_url": null, "username": null, "phone_number": null,
            "public_metadata": {}, "label": null,
            "verification": sdk_verification(Some("oauth_apple"), Value::Null)
        }
    ]);

    let session = json!({
        "object": "session", "id": "sess_snapshot", "status": "active",
        "expire_at": 1731932703435i64, "abandon_at": 1733919903435i64,
        "last_active_at": 1731327903492i64,
        "last_active_token": {"object": "token", "jwt": "synthetic.header.signature"},
        "actor": null, "tasks": null, "last_active_organization_id": null, "user": user,
        "public_user_data": {"first_name": null, "last_name": null, "image_url": null, "has_image": null},
        "factor_verification_age": [0, 1],
        "created_at": 1731327798987i64, "updated_at": 1731327903492i64
    });

    let mut snapshot = json!({
        "object": "client", "id": "client_snapshot", "sessions": [session],
        "sign_in": null, "sign_up": null, "last_active_session_id": "sess_snapshot",
        "cookie_expires_at": null, "captcha_bypass": false, "last_authentication_strategy": null,
        "created_at": 1731327798987i64, "updated_at": 1731327903492i64
    });
    snapshot["sign_in"] = json!({
        "object": "sign_in", "id": "sign_in_snapshot", "status": null,
        "supported_identifiers": [], "supported_first_factors": null, "supported_second_factors": null,
        "first_factor_verification": sdk_verification(Some("oauth_google"), Value::Null),
        "second_factor_verification": sdk_verification(Some("oauth_apple"), Value::Null),
        "identifier": null, "user_data": {"image_url": null, "has_image": null}, "created_session_id": null, "protect_check": null
    });
    snapshot["sign_up"] = json!({
        "object": "sign_up", "id": "sign_up_snapshot", "status": null,
        "required_fields": [], "optional_fields": [], "missing_fields": [], "unverified_fields": [],
        "verifications": {
            "email_address": {"next_action": "needs_prepare", "supported_strategies": ["email_code"], "object": "verification", "id": "", "status": "unverified", "strategy": "email_code", "nonce": null, "message": null, "external_verification_redirect_url": null, "attempts": null, "expire_at": null, "error": null, "verified_at_client": null},
            "phone_number": null, "web3_wallet": null,
            "external_account": sdk_verification(Some("oauth_apple"), Value::Null)
        },
        "username": null, "email_address": null, "phone_number": null, "web3_wallet": null,
        "has_password": false, "first_name": null, "last_name": null,
        "unsafe_metadata": {}, "created_session_id": null, "created_user_id": null,
        "abandon_at": null, "legal_accepted_at": null, "locale": null,
        "external_account": null, "external_account_strategy": null, "protect_check": null
    });

    let client: ClientClient = serde_json::from_value(snapshot).unwrap();
    assert_eq!(client.id, "client_snapshot");
    assert_eq!(
        client.last_active_session_id.as_deref(),
        Some("sess_snapshot")
    );
    assert_eq!(client.created_at, Some(1731327798987i64));
    assert_eq!(client.updated_at, Some(1731327903492i64));
    assert_eq!(client.sessions[0].id, "sess_snapshot");
    assert_eq!(client.sessions[0].last_active_at, 1731327903492i64);
    assert_eq!(
        client.sessions[0]
            .last_active_token
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
            .jwt,
        "synthetic.header.signature"
    );
    assert_eq!(client.sign_in.as_ref().unwrap().status, None);
    assert_eq!(client.sign_up.as_ref().unwrap().status, None);
    assert!(matches!(
        client.sign_in.as_ref().unwrap().object,
        clerk_fapi_rs::models::client_sign_in::Object::SignIn
    ));
    assert!(matches!(
        client.sign_up.as_ref().unwrap().object,
        clerk_fapi_rs::models::client_sign_up::Object::SignUp
    ));
    assert_eq!(
        serde_json::to_value(&client).unwrap()["sign_up"]["has_password"],
        false
    );
    let user = client.sessions[0].user.as_ref().unwrap();
    assert_eq!(user.created_at, None);
    assert_eq!(user.updated_at, None);
    let Some(ClientEmailAddressVerification::ClientSdkVerification(email)) =
        user.email_addresses[0].verification.as_deref()
    else {
        panic!("email verification should retain the SDK snapshot fields");
    };
    assert_eq!(
        email.error.as_ref().unwrap().code,
        "form_identifier_not_found"
    );
    assert_eq!(email.verified_at_client, None);
    assert_eq!(user.email_addresses[0].reserved, None);
    assert_eq!(user.email_addresses[0].created_at, None);
    assert_eq!(user.external_accounts[0].created_at, None);
    assert!(matches!(
        user.passkeys[0].verification.as_deref(),
        Some(ClientPasskeyVerification::ClientSdkVerification(_))
    ));
    assert!(matches!(
        user.external_accounts[0].verification.as_deref(),
        Some(ExternalAccountWithVerificationVerification::ClientSdkVerification(_))
    ));
    let Some(ExternalAccountWithVerificationVerification::ClientSdkVerification(google)) =
        user.external_accounts[0].verification.as_deref()
    else {
        panic!("Google verification should retain the SDK snapshot fields");
    };
    assert_eq!(google.strategy.as_deref(), Some("oauth_google"));
    assert!(matches!(
        google.status,
        Some(clerk_fapi_rs::models::client_sdk_verification::Status::Verified)
    ));
    assert_eq!(
        google.verified_at_client.as_deref(),
        Some("client_snapshot")
    );
    assert!(matches!(
        client
            .sign_in
            .as_ref()
            .unwrap()
            .first_factor_verification
            .as_deref(),
        Some(ClientSignInFirstFactorVerification::ClientSdkVerification(
            _
        ))
    ));
    assert!(matches!(
        client
            .sign_up
            .as_ref()
            .unwrap()
            .verifications
            .email_address
            .as_deref(),
        Some(ClientSignUpVerification::ClientSdkVerification(_))
    ));
}

#[test]
fn generic_verification_rejects_unknown_fields_and_invalid_field_types() {
    let mut unsupported_field = sdk_verification(None, Value::Null);
    unsupported_field["unsupported"] = json!(true);
    assert!(serde_json::from_value::<ClientSdkVerification>(unsupported_field).is_err());

    let mut invalid_status = sdk_verification(None, Value::Null);
    invalid_status["status"] = json!(42);
    assert!(serde_json::from_value::<ClientSdkVerification>(invalid_status).is_err());

    let mut invalid_object = sdk_verification(None, Value::Null);
    invalid_object["object"] = json!("verification_unknown");
    assert!(serde_json::from_value::<ClientSdkVerification>(invalid_object).is_err());

    let mut unknown_error_meta = sdk_verification(
        None,
        json!({"code": "error", "message": "failed", "meta": {"unknown": true}}),
    );
    unknown_error_meta["status"] = json!("failed");
    assert!(serde_json::from_value::<ClientSdkVerification>(unknown_error_meta).is_err());
}

#[test]
fn legacy_external_account_verifications_keep_their_discriminator() {
    let oauth = json!({
        "object": "verification_oauth", "status": "verified", "strategy": "from_oauth_google",
        "external_verification_redirect_url": null, "error": null, "expire_at": 0,
        "attempts": null, "verified_at_client": null
    });
    let one_tap = json!({
        "object": "verification_google_one_tap", "status": "verified", "strategy": "google_one_tap",
        "expire_at": null, "attempts": null, "verified_at_client": null, "error": null
    });

    for legacy in [oauth, one_tap] {
        let parsed: ExternalAccountWithVerificationVerification =
            serde_json::from_value(legacy.clone()).unwrap();
        assert_eq!(
            serde_json::to_value(parsed).unwrap()["object"],
            legacy["object"]
        );
    }

    assert!(
        serde_json::from_value::<ExternalAccountWithVerificationVerification>(json!({
            "status": "verified", "strategy": "from_oauth_google", "expire_at": 0
        }))
        .is_err()
    );
    assert!(serde_json::from_value::<ExternalAccountWithVerificationVerification>(json!({
        "object": "verification_unknown", "status": "verified", "strategy": "from_oauth_google", "expire_at": 0
    }))
    .is_err());
}
