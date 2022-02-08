use actix_web::{HttpRequest, Responder};

#[get("/")]
pub async fn index(_req: HttpRequest) -> impl Responder {
    r"<h1>Hello,This is a Jrebel & JetBrains License Server!</h1>"
}
