use actix_web::{post, get, web::{self, Json}, App, HttpServer, Result, Responder, middleware};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Greeting {
    firstname: String,
    lastname: String
}

// curl -X POST -H "Content-Type: application/json" -d @json/greeting.json 127.0.0.1:8080/greeting
#[post("/greeting")]
async fn greet(greeting: web::Json<Greeting>) -> String {
    format!("Greetings, {}!", greeting.firstname)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
        .wrap(middleware::Logger::default())
        .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}