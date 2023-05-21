use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, HttpRequest};

#[get("/")]
async fn hello(req: HttpRequest) -> impl Responder {
    // HttpResponse::Ok().body(format!("{}", req.connection_info().peer_addr().unwrap()))
    HttpResponse::Ok().body(req.connection_info().peer_addr().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello)
    }).bind(("127.0.0.1", 8080))?
        .run().await
}