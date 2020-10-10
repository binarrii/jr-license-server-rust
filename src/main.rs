use actix_web::{middleware, App, HttpServer};

mod config;
mod handler;

#[actix_web::main]
#[cfg(unix)]
async fn main() -> std::io::Result<()> {
    ::std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .configure(config::app::config_services)
    })
    .bind_uds("/tmp/actix-uds.socket")?
    .run()
    .await
}

#[cfg(not(unix))]
fn main() -> std::io::Result<()> {
    println!("not supported");
    Ok(())
}