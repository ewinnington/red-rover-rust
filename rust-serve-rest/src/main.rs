#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate lazy_static;

use std::{env, io};
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};

mod constants; 
mod response;
mod event;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // 
            .route("/", web::get().to(index))
            // register HTTP requests handlers 
            // Service is used to indicate the routing is done by attribute
            // on the function itself 
            .service(event::list)
            .service(event::get)
            .service(event::create)
            .service(event::delete)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}