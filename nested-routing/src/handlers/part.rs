use actix_web::{Error, HttpResponse, web};

use crate::data::{Part, Product};

pub async fn get_parts(query: web::Query<Option<Part>>) -> Result<HttpResponse, Error> {
    println!("{:?}",query);
    Ok(HttpResponse::Ok().finish())
}
  println!("{:?}",query);
pub async fn add_part(new_part: web::Json<Product>) -> Result<HttpResponse, Error> {
      println!("{:?}",new_part);
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_part_detail(id: web::Path<String>) -> Result<HttpResponse, Error> {
    println!("{:?}",id);
    Ok(HttpResponse::Ok().finish())
}

pub async fn remove_part(_id: web::Path<String>) -> Result<HttpResponse, Error> {
    println!("{:?}",_id);
    Ok(HttpResponse::Ok().finish())
}