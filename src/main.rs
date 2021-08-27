mod api;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};

#[post("/users")]
async fn create_user(request: web::Json<api::create_user::Request>) -> impl Responder {
    match api::create_user::handle(request.0).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::Ok().body(format!("{}", e)),
    }
}

#[post("/recover-user")]
async fn recover_user(request: web::Json<api::recover_user::Request>) -> impl Responder {
    match api::recover_user::handle(request.0).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::Ok().body(format!("{}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(create_user).service(recover_user))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
