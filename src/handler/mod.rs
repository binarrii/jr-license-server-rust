use serde::{Serialize, Deserialize};

#[macro_use]
pub mod macros {
    #[macro_export]
    macro_rules! success {
        ($data:expr) => {
            crate::handler::Resp { code: 0, msg: "success", data: Some($data) }
        };
    }

    #[macro_export]
    macro_rules! failure {
        ($msg:expr) => {
            crate::handler::Resp::<()> { code: -1, msg: $msg, data: None }
        };
    }
}

pub mod index_handler;
pub mod leases_handler;
pub mod ticket_handler;
pub mod validate_handler;
pub mod ping_handler;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct FormData {
    product: String,
    username: Option<String>,
    randomness: Option<String>,
    guid: Option<String>,
    offline: Option<bool>,
    clientTime: u64,
    clientProtocolVersion: String,
    signature: String,
    rebelVersion: String,
    definedUserId: String,
    token: String,
    salt: Option<String>,
}

#[derive(Serialize)]
pub struct Resp<T: Serialize> {
    code: i8,
    msg: &'static str,
    data: Option<T>,
}
