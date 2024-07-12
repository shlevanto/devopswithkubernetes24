use std::env;
use dotenv::dotenv;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Running.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let port: u16 = env::var("PORT").unwrap().parse::<u16>().unwrap();
    
    println!("Server started in port {}", port);

    HttpServer::new(|| { 
        App::new()
        .service(home)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await

}
