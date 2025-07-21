

use std::io::Result;

use actix_files::NamedFile;
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{get, http::{header::ContentType, StatusCode}, middleware, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};

#[get("/facvicon")]
async fn facvicon() -> Result<impl Responder> {
   Ok(NamedFile::open("static/favicon.png"))
}

#[get("/welcome")]
async fn welcome(req : HttpRequest,session : Session) -> Result<impl Responder> {
    println!("{req:?}");

    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("Session value : {counter}");
        counter = counter + 1;
    };
    session.insert("counter",counter);

    Ok(HttpResponse::build(StatusCode::OK)
    .content_type(ContentType::plaintext())
    .body(include_str!("../static/welcome.html"))
)

}
const SESSION_SIGNING_KEY : &[u8] = &[0; 64];


#[actix_web::main]
async fn main() -> Result<()>{

let key = actix_web::cookie::Key::from(SESSION_SIGNING_KEY);
  
   HttpServer::new(move || {
    App::new()
    .wrap(middleware::Compress::default())
    .wrap(
        SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
        .cookie_secure(false)
        .build(),
    )
    .wrap(
        middleware::Logger::default().log_target("@"))
   })

   .bind(("localhost",8081))?
   .run()
   .await

}