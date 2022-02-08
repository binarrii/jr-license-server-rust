use actix_web::{HttpRequest, HttpResponse};

#[get("/")]
pub async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(r"<h1>Hello,This is a Jrebel & JetBrains License Server!</h1>")
}
