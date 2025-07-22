use actix_web::{ get, post, web::{Data, Json, Path}, HttpResponse, Responder
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::AppState;

#[derive(Serialize, FromRow)]
struct User {
    id: i32,
    firstname: String,
    lastname: String,
}

#[derive(Serialize, FromRow)]
struct Article {
    id: i32,
    title: String,
    content: String,
    created_by: i32,
}

#[derive(Deserialize)]
struct CreateArticleBody {
    title: String,
    content: String,
}

#[derive(Deserialize)]
struct CreateUser {
      firstname: String,
      lastname: String,
}

/// GET /users
#[get("/users")]
pub async fn fetch_users(state: Data<AppState>) -> impl Responder {
    let result = sqlx::query_as::<_, User>(
        "SELECT id, firstname, lastname FROM users"
    )
    .fetch_all(&state.db)
    .await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::NotFound().json("Users not found"),
    }
}

/// GET /users/{id}/articles
#[get("/users/{id}/articles")]
pub async fn fetch_users_articles(
    state: Data<AppState>,
    path: Path<i32>,
) -> impl Responder {
    let user_id = path.into_inner();

    let result = sqlx::query_as::<_, Article>(
        "SELECT id, title, content, created_by FROM articles WHERE created_by = $1"
    )
    .bind(user_id)
    .fetch_all(&state.db)
    .await;

    match result {
        Ok(articles) => HttpResponse::Ok().json(articles),
        Err(_) => HttpResponse::NotFound().json("Articles not found"),
    }
}

/// POST /users/{id}/articles
#[post("/users/{id}/articles")]
pub async fn create_user_article(
    state: Data<AppState>,
    path: Path<i32>,
    body: Json<CreateArticleBody>,
) -> impl Responder {
    let user_id = path.into_inner();

    let result = sqlx::query_as::<_, Article>(
        "INSERT INTO articles (title, content, created_by) 
         VALUES ($1, $2, $3) 
         RETURNING id, title, content, created_by"
    )
    .bind(&body.title)
    .bind(&body.content)
    .bind(user_id)
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create article"),
    }
}

#[post("/users")]
pub async fn create_user(state : Data<AppState>,body : Json<CreateUser>) -> impl Responder {
    let result = sqlx::query_as::<_,User>(
        "INSERT INTO users (firstname , lastname) VALUES ($1,$2) RETURNING id , firstname , lastname"
    )
    .bind(&body.firstname).bind(&body.lastname).fetch_one(&state.db).await;

    match  result {
          Ok(user) => HttpResponse::Ok().json(user),
          Err(_) => HttpResponse::InternalServerError().json("Failed while creating user")        
    }
}