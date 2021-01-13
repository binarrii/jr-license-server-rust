// use actix_web::{middleware, App, HttpServer};
//
// mod config;
// mod handler;
// mod util;
//
// #[actix_web::main]
// // #[cfg(unix)]
// async fn main() -> std::io::Result<()> {
//     ::std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
//     env_logger::init();
//
//     HttpServer::new(|| {
//         App::new()
//             // enable logger - always register actix-web Logger middleware last
//             .wrap(middleware::Logger::default())
//             .configure(config::app::config_services)
//     })
//     .bind("127.0.0.1:8000")?
//     .run()
//     .await
// }

extern crate openssl;

use openssl::hash::MessageDigest;

fn main() {
    let key = std::fs::read_to_string("KEY").expect("Key file not found");
    let der = base64::decode(key.replace("\n", "")).expect("Invalid key file");
    let pkey = openssl::pkey::PKey::private_key_from_der(&der).unwrap();

    let data = "VqlviUKd5es=;H2ulzLlh7E0=;xkkhJCgsum/uYQ2k/E3Kx71vE4t7uOpnMx6eYkdBxjc=;false".as_bytes();

    let mut signer = openssl::sign::Signer::new(MessageDigest::sha1(), pkey.as_ref()).unwrap();
    signer.update(data).unwrap();

    let sign_vec = signer.sign_to_vec().unwrap();
    let sign_str = base64::encode(sign_vec);

    println!("{}", sign_str)
}
