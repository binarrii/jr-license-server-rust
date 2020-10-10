use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

// POST /jrebel/leases
pub async fn lease(params: web::Form<FormData>) -> HttpResponse {
    if let None = params.username {
        return HttpResponse::Forbidden().finish();
    }
    if let None = params.randomness {
        return HttpResponse::Forbidden().finish();
    }
    if let None = params.guid {
        return HttpResponse::Forbidden().finish();
    }

    return HttpResponse::Ok().json(RespData {
        company: params.username.as_deref().unwrap_or("").to_string(),
        evaluationLicense: false,
        groupType: "managed".to_string(),
        id: 1,
        licenseType: 1,
        licenseValidFrom: 1490544001000,
        licenseValidUntil: 1691839999000,
        offline: params.offline.to_string(),
        orderId: "".to_string(),
        seatPoolType: "standalone".to_string(),
        serverGuid: "a1b4aea8-b031-4302-b602-670a990272cb".to_string(),
        serverProtocolVersion: "1.1".to_string(),
        serverRandomness: "H2ulzLlh7E0=".to_string(),
        serverVersion: "3.2.4".to_string(),
        signature: "".to_string(), // TODO
        statusCode: "SUCCESS".to_string(),
        validFrom: None,
        validUntil: None,
        zeroIds: [],
    });
}

#[derive(Deserialize)]
pub struct FormData {
    username: Option<String>,
    randomness: Option<String>,
    guid: Option<String>,
    offline: bool,
    clientTime: String,
}

#[derive(Serialize)]
pub struct RespData {
    company: String,
    evaluationLicense: bool,
    groupType: String,
    id: u32,
    licenseType: u32,
    licenseValidFrom: u64,
    licenseValidUntil: u64,
    offline: String,
    orderId: String,
    seatPoolType: String,
    serverGuid: String,
    serverProtocolVersion: String,
    serverRandomness: String,
    serverVersion: String,
    signature: String,
    statusCode: String,
    validFrom: Option<String>,
    validUntil: Option<String>,
    zeroIds: [String; 0],
}
