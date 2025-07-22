use std::io::Result;

use actix_web::{web::{self, Data}, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::services::{create_user, create_user_article, fetch_users, fetch_users_articles};

mod services;


pub struct AppState {
    db: Pool<Postgres>
}

#[actix_web::main]
async fn main() ->Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("url is no");

    let pool = PgPoolOptions::new()
    .max_connections(4)
    .connect(&database_url)
    .await
    .expect("Error while building a connection pool");

    HttpServer::new(move || {
        App::new()
        .app_data(Data::new(AppState {db : pool.clone()}))
        .service(fetch_users)
        .service(fetch_users_articles)
        .service(create_user_article)
        .service(create_user)
    })
    .bind(("127.0.0.1",8081))?
    .run()
    .await
}
