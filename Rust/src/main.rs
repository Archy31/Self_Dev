#![warn(clippy::all, clippy::pedantic)]

use actix_web::{
    get, App, HttpResponse, HttpServer, Responder
};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new()
        .service(start)
    }).bind(("127.0.0.1", 8080))?
    .run()
    .await
}


#[get("/")]
async fn start() -> impl Responder {
    HttpResponse::Ok().body("All right!")
}