use actix_web::{get, App, HttpServer, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[get("/secret")]
async fn secret() -> impl Responder {
    HttpResponse::Ok().body("This is a secret page.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:3000";
    println!("Listening on {}", addr);

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(secret)
    })
    .bind(addr)?
    .run()
    .await
}

