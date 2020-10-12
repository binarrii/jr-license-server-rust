use actix_web::web::Data;
use serde::Serialize;
use serde_json::json;

pub mod index_handler;
pub mod leases_handler;
pub mod ticket_handler;
pub mod validate_handler;
pub mod ping_handler;

#[derive(Serialize)]
pub struct Resp<T: Serialize> {
    code: i8,
    msg: &'static str,
    data: Option<T>,
}

impl<T> Resp<T> where T: Serialize {
    fn success(d: T) -> Self {
        Resp { code: 0, msg: "ok", data: Some(d) }
    }

    fn failure(s: &'static str) -> Self {
        Resp { code: -1, msg: s, data: None }
    }
}
