use actix_web::{post, get, web::{self, Json}, App, HttpServer, error, Result as WebResult, Responder, middleware};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Person {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub age: u32,
}

#[get("/bond/")]
async fn get_mr_bond() -> Json<Person> {
    todo!()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
        .wrap(middleware::Logger::default())
        .service(get_mr_bond)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
