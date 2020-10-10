use crate::handler::*;
use actix_web::web;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    // info!("Configurating routes...");
    cfg.service(web::resource("/").route(web::get().to(index_handler::index)))
        .service(
            web::scope("/jrebel")
                .service(
                    web::resource("/leases")
                        .route(web::post().to(leases_handler::lease))
                )
                .service(
                    web::resource("/leases/1")
                        .route(web::post().to(leases_handler::lease))
                )
                .service(
                    web::resource("/validate-connection")
                        .route(web::post().to(leases_handler::lease))
                )
        )
        .service(
            web::scope("/agent")
                .service(
                    web::resource("/leases")
                        .route(web::post().to(leases_handler::lease))
                )
                .service(
                    web::resource("/leases/1")
                        .route(web::post().to(leases_handler::lease))
                )
        )
        .service(
            web::scope("/rpc")
                .service(
                    web::resource("/obtainTicket.action")
                        .route(web::post().to(leases_handler::lease))
                )
                .service(
                    web::resource("/releaseTicket.action")
                        .route(web::post().to(leases_handler::lease))
                )
                .service(
                    web::resource("/ping.action")
                        .route(web::post().to(leases_handler::lease))
                )
        );
    
}
