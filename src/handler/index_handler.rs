use actix_web::HttpRequest;

#[get("/")]
pub async fn index(_req: HttpRequest) -> &'static str {
    "Hello, This is a Jrebel License Server !"
}
