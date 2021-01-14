use std::ops::Deref;

use actix_web::{HttpResponse, web};

use crate::handler::FormData;
use crate::util::sign;

#[post("/rpc/obtainTicket.action")]
pub async fn obtain_ticket(params: web::Form<FormData>) -> HttpResponse {
    match params.deref() {
        FormData { userName: None, .. } => HttpResponse::Forbidden().finish(),
        FormData { salt: None, .. } => HttpResponse::Forbidden().finish(),
        FormData { .. } => {
            let salt = params.salt.as_deref().unwrap();
            let uname = params.userName.as_deref().unwrap();

            let data = format!("<ObtainTicketResponse><message></message><prolongationPeriod>607875500</prolongationPeriod><responseCode>OK</responseCode><salt>{}</salt><ticketId>1</ticketId><ticketProperties>licensee={}\tlicenseType=0\t</ticketProperties></ObtainTicketResponse>", salt, uname);
            let sign = sign::sign_with_asn1_key(data.clone());
            let resp = format!("<!-- {} -->\n{}", sign, data);

            HttpResponse::Ok().content_type("text/html;charset=utf-8").body(resp)
        }
    }
}

#[post("/rpc/releaseTicket.action")]
pub async fn release_ticket(params: web::Form<FormData>) -> HttpResponse {
    if let Some(salt) = &params.salt {
        let data = format!("<ReleaseTicketResponse><message></message><responseCode>OK</responseCode><salt>{}</salt></ReleaseTicketResponse>", salt);
        let sign = sign::sign_with_asn1_key(data.clone());
        let resp = format!("<!-- {} -->\n{}", sign, data);

        HttpResponse::Ok().content_type("text/html;charset=utf-8").body(resp)
    } else {
        HttpResponse::Forbidden().finish()
    }
}
