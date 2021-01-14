use actix_web::{HttpResponse, web};

use crate::handler::FormData;
use crate::util::sign;

#[post("/rpc/ping.action")]
pub async fn ping(params: web::Form<FormData>) -> HttpResponse {
    if let Some(salt) = &params.salt {
        let data = format!(
            r#"
            <PingResponse>
                <message></message>
                <responseCode>OK</responseCode>
                <salt>{}</salt>
            </PingResponse>
            "#, salt);

        let sign = sign::sign_with_asn1_key(data.clone());
        let resp = format!("<!-- {} -->\n{}", sign, data);

        HttpResponse::Ok().body(resp)
    } else {
        HttpResponse::Forbidden().finish()
    }
}