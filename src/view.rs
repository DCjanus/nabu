use actix_web::Responder;

#[get("/ping")]
pub async fn ping() -> impl Responder {
    "pong"
}
