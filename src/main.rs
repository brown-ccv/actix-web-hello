use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn greet(name: web::Path<String>) -> impl Responder {
    let greeting = format!("Hello, {}", name);
    HttpResponse::Ok().body(greeting)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = "8080";
    let ip_address = format!("0.0.0.0:{}", port);

    println!("Listening on port: {}", port);

    HttpServer::new(|| {
        App::new()
            .route("/{name}", web::get().to(greet))
    })
    .bind(ip_address)?
    .run()
    .await
}

