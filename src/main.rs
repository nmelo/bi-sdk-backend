mod api;
mod models;
mod utils;

use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::env;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Beyond Identity SDK Backend")
}

#[post("/users")]
async fn create_user(request: web::Json<api::create_user::Request>) -> impl Responder {
    match api::create_user::handle(request.0).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::from_error(e),
    }
}

#[post("/recover-user")]
async fn recover_user(request: web::Json<api::recover_user::Request>) -> impl Responder {
    match api::recover_user::handle(request.0).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::from_error(e),
    }
}

#[post("/auth-user")]
async fn auth_user(request: web::Json<api::auth_user::Request>) -> impl Responder {
    match api::auth_user::handle(request.0).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::from_error(e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(index)
            .service(create_user)
            .service(recover_user)
            .service(auth_user)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
