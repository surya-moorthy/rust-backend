use std::io::Result;

use actix_web::{middleware, App, HttpServer};
use nested_routing::config::app_config;


#[actix_web::main]
async fn main() -> Result<()> {
    
    HttpServer::new( || {
        App::new()
        .wrap(middleware::Logger::default())
        .configure(app_config)
    }   
    ).bind("localhost:8081")?
    .run()
    .await
}
