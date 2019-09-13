use actix_web::{web, App, HttpServer, Responder};

fn index(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

fn main() {
    run("127.0.0.1:13370");
}

fn run(host: &'static str) {
    HttpServer::new(|| App::new().service(web::resource("/{name}").to(index)))
        .bind(host)
        .expect(&format!("Host: {} is disabled", host))
        .run()
        .expect("Can't running HTTP Server");
}
