use actix_web::{get,post, web::{self, Json}, App, HttpServer, middleware};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct Person {
    pub id: u32,
    pub first_name: &'static str,
    pub last_name: &'static str,
    pub age: u32,
}

#[derive( Deserialize)]
struct PathInfo{
    id: u32
}

#[get("/person/{user_id}")]
async fn get_person_by_id(path: web::Path<PathInfo>) -> Option<Json<Person>> {
    // TODO...
    todo!()
}

#[get("/person/")]
async fn list_person() -> Json<Vec<Person>> {
    // TODO...
    todo!()
}

#[post("/person/")]
async fn post_person(path: web::Path<PathInfo>) -> Option<Json<Person>> {
    // TODO...
    todo!()
}

#[post("/person/")]
async fn delete_person(path: web::Path<PathInfo>) -> Option<Json<Person>> {
    // TODO...
    todo!()
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


