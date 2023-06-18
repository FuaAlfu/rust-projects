use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[get("/users")]
async fn get_users() -> impl Responder {
    HttpResponse::Ok().json(vec![
        User {
            id: 1,
            name: "John".to_string(),
            email: "john@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Jane".to_string(),
            email: "jane@example.com".to_string(),
        },
    ])
}

#[post("/users")]
async fn create_user(user: web::Json<User>) -> impl Responder {
    HttpResponse::Created().json(user.into_inner())
}

#[put("/users/{id}")]
async fn update_user(web::Path(id): web::Path<u32>, user: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().json(User {
        id,
        ..user.into_inner()
    })
}

#[delete("/users/{id}")]
async fn delete_user(web::Path(id): web::Path<u32>) -> impl Responder {
    HttpResponse::NoContent().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_users).service(create_user).service(update_user).service(delete_user))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}