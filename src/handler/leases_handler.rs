use std::ops::Deref;

use actix_web::{HttpResponse, web};
use serde_json::json;

use crate::handler::FormData;

const SERVER_RAND: &str = "H2ulzLlh7E0=";

#[post("/jrebel/leases")]
pub async fn jrebel_lease(params: web::Form<FormData>) -> HttpResponse {
    handle_lease(params)
}

#[post("/agent/leases")]
pub async fn agent_lease(params: web::Form<FormData>) -> HttpResponse {
    handle_lease(params)
}

fn handle_lease(params: web::Form<FormData>) -> HttpResponse {
    match params.deref() {
        FormData { username: None, .. } =>
            HttpResponse::Forbidden().finish(),
        FormData { randomness: None, .. } =>
            HttpResponse::Forbidden().finish(),
        FormData { guid: None, .. } =>
            HttpResponse::Forbidden().finish(),
        FormData { .. } => {
            let offline = params.offline.unwrap_or(false);
            let validity = gen_validity(offline, params.clientTime);
            let client_rand = params.randomness.as_deref().unwrap_or("");
            let guid = params.guid.as_deref().unwrap_or("");
            let sign = sign(offline, client_rand, guid, &validity);

            let resp = json!({
                "company": params.username,
                "evaluationLicense": false,
                "groupType": "managed",
                "id": 1,
                "licenseType": 1,
                "licenseValidFrom": 1490544001000u64,
                "licenseValidUntil": 1691839999000u64,
                "offline": offline,
                "orderId": "",
                "seatPoolType": "standalone",
                "serverGuid": "a1b4aea8-b031-4302-b602-670a990272cb",
                "serverProtocolVersion": "1.1",
                "serverRandomness": String::from(SERVER_RAND),
                "serverVersion": "3.2.4",
                "signature": sign,
                "statusCode": "SUCCESS",
                "validFrom": validity.0,
                "validUntil": validity.1,
                "zeroIds": []
            });
            HttpResponse::Ok().json(resp)
        }
    }
}

#[post("/jrebel/leases/1")]
pub async fn jrebel_lease_1(params: web::Form<FormData>) -> HttpResponse {
    handel_lease_1(params)
}

#[post("/agent/leases/1")]
pub async fn agent_lease_1(params: web::Form<FormData>) -> HttpResponse {
    handel_lease_1(params)
}

fn handel_lease_1(params: web::Form<FormData>) -> HttpResponse {
    let resp = json!({
        "company": params.username,
        "serverVersion": "3.2.4",
        "serverProtocolVersion": "1.1",
        "serverGuid": "a1b4aea8-b031-4302-b602-670a990272cb",
        "groupType": "managed",
        "statusCode": "SUCCESS",
        "msg": None::<String>,
        "statusMessage": None::<String>
    });
    HttpResponse::Ok().json(resp)
}

fn gen_validity(offline: bool, client_time: u64) -> (Option<String>, Option<String>) {
    let valid_from = client_time;
    let valid_until = valid_from + 180 * 24 * 60 * 60 * 1000;

    if offline {
        (Some(valid_from.to_string()), Some(valid_until.to_string()))
    } else {
        (None, None)
    }
}

fn sign(offline: bool, client_rand: &str, guid: &str,
        validity: &(Option<String>, Option<String>)) -> String {
    let data = if offline {
        format!("{};{};{};{};{};{}", client_rand, SERVER_RAND, guid, offline,
                validity.0.as_deref().unwrap_or(""),
                validity.1.as_deref().unwrap_or(""))
    } else {
        format!("{};{};{};{}", client_rand, SERVER_RAND, guid, offline)
    };

    crate::util::sign::sign_with_pkcs8_key(data)
}
