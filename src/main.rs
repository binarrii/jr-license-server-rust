#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate lazy_static;

use actix_web::{App, HttpServer, middleware};

use handler::{index_handler, leases_handler, ticket_handler, validate_handler};

mod config;
mod handler;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    ::std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index_handler::index)
            .service(leases_handler::jrebel_lease)
            .service(leases_handler::jrebel_lease_1)
            .service(leases_handler::agent_lease)
            .service(leases_handler::agent_lease_1)
            .service(ticket_handler::obtain_ticket)
            .service(ticket_handler::release_ticket)
            .service(validate_handler::validate)
    })
    .bind("127.0.0.1:10017")?
    .run()
    .await
}
