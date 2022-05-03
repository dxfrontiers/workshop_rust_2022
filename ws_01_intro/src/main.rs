use actix_web::{post, get, web, App, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Greeting {
    name: String,
}

// curl 127.0.0.1:8080/hello
#[get("/hello_str")]
async fn hello_str() -> &'static str {
    "Hello there"
}

// curl 127.0.0.1:8080/hello
#[get("/hello")]
async fn hello() -> String {
    String::from("Hello there")
}

// curl -v "127.0.0.1:8080/greeting?name=GeneralKenobi"
#[get("/greeting")]
async fn greet_query(greeting: web::Query<Greeting>) -> String {
    format!("Hello There  {}!", greeting.name)
}

// curl -X POST -H 'Content-Type: application/json' -d '{"name":"test123"}' 127.0.0.1:8080/greeting
#[post("/greeting")]
async fn greet(greeting: web::Json<Greeting>) -> String {
    format!("Greetings, {}!", greeting.name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(hello)
        .service(hello_str)
        .service(greet_query)
        .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}