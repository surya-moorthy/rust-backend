use actix_web::{web, App, HttpServer, Responder};


async fn hello() -> impl Responder {
    "Hello World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/app").route("/index.html", web::get().to(hello))
        )
    }).bind(("localhost",8081))?.run().await
}


// web::scope is similar to the app.use("/",_) in ts