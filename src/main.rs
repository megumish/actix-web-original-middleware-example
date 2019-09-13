use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, Responder};

fn index(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

fn main() {
    run("127.0.0.1:13370");
}

fn run(host: &'static str) {
    // on Logger level info
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(web::resource("/{name}").to(index))
    })
    .bind(host)
    .expect(&format!("Host: {} is disabled", host))
    .run()
    .expect("Can't running HTTP Server");
}
