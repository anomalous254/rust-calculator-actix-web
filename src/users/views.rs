use actix_web::{get,web, HttpResponse, Responder};
use crate::models::users::{CalcQuery, CalcResponse};
use actix_files::NamedFile;
use std::path::PathBuf;


#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open(PathBuf::from("static/index.html"))
}


#[get("/calculate")]
async fn calculator(query: web::Query<CalcQuery>) -> impl Responder {
    let valid_operations: Vec<&str> = vec!["add", "sub", "mul", "div"];

    let result: f64 = match query.operation.as_str() {
        "add" => query.num1 + query.num2,
        "sub" => query.num1 - query.num2,
        "mul" => query.num1 * query.num2,
        "div" => {
            if query.num2 == 0.0 {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Cannot divide by zero"
                }));
            }
            query.num1 / query.num2
        },
        _ => return HttpResponse::BadRequest().json(serde_json::json!({
            "valid_operations": valid_operations,
            "error": "Invalid operation"
        }))
    };

    HttpResponse::Ok().json(CalcResponse{result})
}


pub fn calc_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index) // Serves `/`
      .service(
          web::scope("/api") // API routes under `/api`
              .service(calculator)
      );
}
