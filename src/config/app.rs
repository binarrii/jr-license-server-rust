use actix_web::web;

use crate::handler::*;

pub fn config_services(scfg: &mut web::ServiceConfig) {
    scfg.route("/", web::get().to(index_handler::index))
        .route("/jrebel/leases", web::post().to(leases_handler::lease))
        .route("/jrebel/leases/1", web::post().to(leases_handler::lease))
        .route("/jrebel/validate-connection", web::post().to(leases_handler::lease))
        .route("/agent/leases", web::post().to(leases_handler::lease))
        .route("/agent/leases/1", web::post().to(leases_handler::lease))
        .route("/rpc/obtainTicket.action", web::post().to(leases_handler::lease))
        .route("/rpc/releaseTicket.action", web::post().to(leases_handler::lease))
        .route("/rpc/ping.action", web::post().to(leases_handler::lease));
}
