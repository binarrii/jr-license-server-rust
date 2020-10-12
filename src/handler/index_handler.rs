use actix_web::HttpRequest;

pub async fn index(_req: HttpRequest) -> &'static str {
    r"
    <h1>Hello,This is a Jrebel & JetBrains License Server!</h1>
    <p>License Server started at</p>;
    <p>JRebel 7.1 and earlier version Activation address was: <span style='color:red'></span>, with any email.</p>
    <p>JRebel 2018.1 and later version Activation address was: , with any email.</p>
    "
}
