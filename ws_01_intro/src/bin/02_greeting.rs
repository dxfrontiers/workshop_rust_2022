use actix_web::{post, get, web::{self, Query}, App, HttpServer, Result, Responder, middleware};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Greeting {
    name: String
}

// curl -s "127.0.0.1:8080/greeting?name=General%20Kenobi"
#[get("/greeting")]
async fn greet_query(greeting: Query<Greeting>) -> String {
    format!("Hello there, {}!\n", greeting.name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));


    HttpServer::new(move || {
        App::new()
        .wrap(middleware::Logger::default())
        .service(greet_query)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}