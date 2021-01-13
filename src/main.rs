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

    let result = openssl::pkey::PKey::private_key_from_der(&der);
    match result {
        Ok(pkey) => {
            let data = "VqlviUKd5es=;H2ulzLlh7E0=;xkkhJCgsum/uYQ2k/E3Kx71vE4t7uOpnMx6eYkdBxjc=;false".as_bytes();

            let mut  signer = openssl::sign::Signer::new(MessageDigest::sha1(), pkey.as_ref()).unwrap();
            signer.update(data).unwrap();
            let signature = signer.sign_to_vec().unwrap();
            let signstr = base64::encode(signature);

            println!("{}", signstr)
        },
        Err(e) => e.errors().iter().for_each(|x| println!("{}", x))
    }

}
