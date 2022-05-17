use actix_web::{post, get, web::{self, Json}, App, HttpServer, Result, Responder, middleware};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Greeting {
    firstname: String,
    lastname: String
}

async fn greet_upper_case(greeting: web::Json<Greeting>) -> String {
    format!("Greetings, {}!", greeting.firstname.to_uppercase())
}

async fn greet_with_length(greeting: web::Json<Greeting>) -> String {
    let greeting = greeting.into_inner();
    let length = get_length_of_name(/*TODO*/&greeting);
    format!("Greetings, {} {}!, your name has {} chars.", greeting.firstname, greeting.lastname, length)
}


fn get_length_of_name(/*TODO*/greeting: &Greeting) -> usize{
    // greeting.firstname.chars().count() + greeting.lastname.chars().count()
    // greeting.firstname.len() + greeting.lastname.len()
    // TODO
    todo!()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
        .wrap(middleware::Logger::default())
        .service(web::resource("/upper").route(web::post().to(greet_upper_case)))
        .service(web::resource("/greet").route(web::post().to(greet_with_length)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


