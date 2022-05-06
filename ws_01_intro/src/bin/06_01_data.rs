use actix_web::{get, web::{self, Json}, App, HttpServer, middleware};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct Person {
    pub id: u32,
    pub first_name: &'static str,
    pub last_name: &'static str,
    pub age: u32,
}

static PEOPLE_STATIC: [Person;2] = 
    [
        Person{
            id: 0,
            last_name: "Peter",
            first_name: "Lustig",
            age: 30,
        },
        Person{
            id: 1,
            last_name: "Hans",
            first_name: "Dampf",
            age: 42,
        },
    ];



#[get("/person/{user_id}")]
async fn get_person_by_id(path: web::Path<PathInfo>) -> Option<Json<Person>> {
    PEOPLE_STATIC
        .iter()
        .filter(|p|p.id==path.id)
        .next()
        .map(|p| p.clone())
        .map(Json)
        // achtung, "owned" version funktioniert nicht mehr
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



#[derive( Deserialize)]
struct PathInfo{
    id: u32
}