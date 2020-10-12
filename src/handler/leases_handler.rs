use std::ops::Deref;

use actix_web::{HttpResponse, web};
use serde::Deserialize;
use serde_json::json;

use crate::handler::Resp;

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct FormData {
    username: Option<String>,
    randomness: Option<String>,
    guid: Option<String>,
    offline: bool,
    clientTime: String,
}

// POST /jrebel/leases
pub async fn lease(params: web::Form<FormData>) -> HttpResponse {
    match params.deref() {
        FormData { username: None, .. } =>
            HttpResponse::Ok().json(Resp::<()>::failure("username can not be null")),
        FormData { randomness: None, .. } =>
            HttpResponse::Ok().json(Resp::<()>::failure("randomness can not be null")),
        FormData { guid: None, .. } =>
            HttpResponse::Ok().json(Resp::<()>::failure("guid can not be null")),
        FormData { .. } =>
            HttpResponse::Ok().json(json!({
                "company": params.username,
                "evaluationLicense": false,
                "groupType": "managed",
                "id": 1,
                "licenseType": 1,
                "licenseValidFrom": 1490544001000u64,
                "licenseValidUntil": 1691839999000u64,
                "offline": params.offline,
                "orderId": "",
                "seatPoolType": "standalone",
                "serverGuid": "a1b4aea8-b031-4302-b602-670a990272cb",
                "serverProtocolVersion": "1.1",
                "serverRandomness": "H2ulzLlh7E0=",
                "serverVersion": "3.2.4",
                "signature": "", // TODO
                "statusCode": "SUCCESS",
                "validFrom": None::<String>,
                "validUntil": None::<String>,
                "zeroIds": []
            })),
    }
}
