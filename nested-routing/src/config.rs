use actix_web::web::{self, ServiceConfig};

use crate::handlers::{part::{add_part, get_part_detail, get_parts, remove_part}, products};



pub async fn app_config(cfg : &mut ServiceConfig) {
    cfg.service(
        web::scope("products")
        .service(
            web::resource("")
            .route(web::get().to(products::get_products))
            .route(web::post().to(products::add_product))
        )
        .service(
            web::scope("/parts")
            .service(
                web::resource("")
                .route(web::get().to(get_parts))
                .route(web::post().to(add_part))
            )
            .service(
                web::resource("/{part_id}")
                .route(web::get().to(get_part_detail))
                .route(web::delete().to(remove_part))
            )
        )
    );
}