use std::{convert::Infallible, io::Result as IoResult};

use actix_files::NamedFile;
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{get, http::{header::ContentType, StatusCode}, middleware, web::{self}, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use actix_web_lab::extract::Path;
use async_stream::stream;

// async_stream, actix_web_lab,actix_web,actix_files

static SESSION_SIGNING_KEY: &[u8] = &[0; 64];

#[get("/favicon")]
async fn favicon() -> Result<impl Responder> {
    Ok(NamedFile::open("static/favicon.ico"))
}

#[get("/welcome")]
async fn welcome(req: HttpRequest,session : Session) -> Result<HttpResponse> {

    let mut counter = 1;

    if let Some(count) = session.get::<i32>("counter")? {
        println!("count :{}",count);
        counter = counter + 1;
        session.insert("counter",count)?;
    }

    session.insert("counter", counter)?;
    
    Ok(HttpResponse::build(StatusCode::OK)
      .content_type(ContentType::plaintext())
      .body(include_str!("../static/welcome.html"))
)
}




async fn streaming_response(path : web::Path<String>) -> HttpResponse {
   let name = path.into_inner();

   HttpResponse::Ok().content_type(ContentType::plaintext()).streaming(stream! {
            yield Ok::<_, Infallible>(web::Bytes::from("Hello "));
            yield Ok::<_, Infallible>(web::Bytes::from(name));
            yield Ok::<_, Infallible>(web::Bytes::from("!"));
   })
}

async fn with_param(req : HttpRequest,Path((name,)) : Path<(String,)>) -> HttpResponse{
     println!("{req:?}");

     HttpResponse::Ok()
     .content_type(ContentType::plaintext())
     .body(format!("Hello {name}"))
}


#[actix_web::main]
async fn main() -> IoResult<()> {

    let key = actix_web::cookie::Key::from(SESSION_SIGNING_KEY);
    
    HttpServer::new( move || {
        App::new()
        .wrap(middleware::Compress::default())
        .wrap(
            SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
            .cookie_name("_cookie_session".to_string())
            .cookie_secure(false)
            .build(),
        )
        .service(favicon)
        .service(welcome)
        .service(web::resource("/user/{name}").route(web::get().to(with_param)))
        .service(web::resource("/async-body/{name}").route(web::get().to(streaming_response)))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}