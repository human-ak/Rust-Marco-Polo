// web microservices for calculating various mathematical operations
// using actix-web framework
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the calculator microservice!")
}

#[get("/add/{a}/{b}")]
async fn add(info: web::Path<(f64, f64)>) -> impl Responder {
    let res = calc::add(info.0, info.1);
    HttpResponse::Ok().body(format!("{} + {} = {}", info.0, info.1, res))
}

#[get("/subtract/{a}/{b}")]
async fn subtract(path: web::Path<(f64, f64)>) -> impl Responder {
    let (a, b) = path.into_inner();
    let result = calc::subtract(a, b);
    HttpResponse::Ok().body(format!("{} - {} = {}", a, b, result))
}

#[get("/multiply/{a}/{b}")]
async fn multiply(path: web::Path<(f64, f64)>) -> impl Responder {
    let (a, b) = path.into_inner();
    let result = calc::multiply(a, b);
    HttpResponse::Ok().body(format!("{} * {} = {}", a, b, result))
}

#[get("/divide/{a}/{b}")]
async fn divide(path: web::Path<(f64, f64)>) -> impl Responder {
    let (a, b) = path.into_inner();
    if b == 0.0 {
        return HttpResponse::BadRequest().body("Cannot divide by zero");
    }
    let result = calc::divide(a, b);
    HttpResponse::Ok().body(format!("{} / {} = {}", a, b, result))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(add)
            .service(subtract)
            .service(multiply)
            .service(divide)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
