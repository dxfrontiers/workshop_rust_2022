use actix_web::{get, web::Json, HttpServer, Responder, App, middleware, body::MessageBody};
use serde::{Serialize, Deserialize};
use actix_web::body::EitherBody;


#[derive(Serialize, Deserialize, Clone)]
pub struct Person {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub age: u32,
}

impl Responder for Person{
    type Body = EitherBody<String>;

    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        Json(self).respond_to(req)
    }
}

#[get("/person/")]
async fn get_person_by_id() -> Person {
    Person{
        id: 007,
        last_name: "Bond".to_string(),
        first_name: "James".to_string(),
        age: 30,
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
        .wrap(middleware::Logger::default())
        .service(get_person_by_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}