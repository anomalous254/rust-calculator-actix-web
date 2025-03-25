use actix_web::{web, HttpResponse, HttpServer, App};
use env_logger;
use actix_web::middleware::Logger;

mod users;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    env_logger::init();

    HttpServer::new(|| {
        App::new()
        .wrap(Logger::default())
        .configure(users::views::calc_routes)
        .default_service(web::route().to(|| async {
                HttpResponse::NotFound().body("404 Not Found")
            }))
        
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await

}