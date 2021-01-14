use actix_web::web;

use crate::handler::*;

#[allow(dead_code)]
pub fn config_services(scfg: &mut web::ServiceConfig) {
    scfg.service(index_handler::index)
        .service(leases_handler::jrebel_lease)
        .service(leases_handler::jrebel_lease_1)
        .service(validate_handler::validate)
    ;
}
