use std::sync::Mutex;

use actix_web::{web, App, HttpServer};

// making this state sharable 
struct AppStateWithCounter {
    counter : Mutex<i32>
}

async fn index(data : web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("the requested number:{counter}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0)
    });

    HttpServer::new(move || {
        App::new().service(
            web::scope("/app").app_data(counter.clone()).route("/count", web::get().to(index))
        )
    }).bind(("localhost",8081))?.run().await
}


// web::scope is similar to the app.use("/",_) in ts