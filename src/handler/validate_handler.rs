use actix_web::{HttpResponse, web};
use serde_json::json;

use crate::handler::FormData;

#[post("/jrebel/validate-connection")]
pub async fn validate(params: web::Form<FormData>) -> HttpResponse {
    let resp = json!({
        "company": params.username,
        "serverVersion": "3.2.4",
        "serverProtocolVersion": "1.1",
        "serverGuid": "a1b4aea8-b031-4302-b602-670a990272cb",
        "groupType": "managed",
        "statusCode": "SUCCESS",
        "canGetLease": true,
        "licenseType": 1,
        "evaluationLicense": false,
        "seatPoolType": "standalone"
    });
    HttpResponse::Ok().json(resp)
}