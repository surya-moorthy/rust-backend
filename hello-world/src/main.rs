use std::io::Result;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
   HttpResponse::Ok().body("Hello World!")
}

#[get("/echo")]
async fn echo(req_body : String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
   HttpResponse::Ok().body("Manual Hello world!")
}

#[actix_web::main]
async fn main() ->Result<()> {
    HttpServer::new(|| {
        App::new().service(hello).service(echo).route("/hey",web::get().to(manual_hello))
    })
    .bind(("127.0.0.1",8081))?
    .run()
    .await
}
