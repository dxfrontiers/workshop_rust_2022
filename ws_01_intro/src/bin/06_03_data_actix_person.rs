use std::{sync::{atomic::AtomicU32, Arc}, collections::HashMap};

use actix_web::{get, web::{self, Json}, App, HttpServer,error, middleware, Responder,  Result as WebResult, HttpResponse};
use serde::{Deserialize, Serialize};

pub struct PersonDB{
    people: HashMap<u32,Person>
}

// uses the actix result type
// #[get("/person/{id}")]
// async fn get_person_by_id(data: web::Data<PersonDB>, path: web::Path<PathInfo>) -> WebResult<impl Responder>{
//     data.people
//         .get(&path.id)    
//         .ok_or(error::ErrorNotFound(format!("Could not find person with id {}",path.id)))
//         .map(serde_json::to_string) 
// }

// uses a hybrid approach
// #[get("/person/{id}")]
// async fn get_person_by_id(data: web::Data<PersonDB>, path: web::Path<PathInfo>) -> Option<impl Responder>{
//     data.people
//         .get(&path.id)
//         .map(serde_json::to_string) 
// }

//uses the explicit type + clone
// #[get("/person/{id}")]
// async fn get_person_by_id(data: web::Data<PersonDB>, path: web::Path<PathInfo>) -> Option<Json<Person>>{
//     data.people
//         .get(&path.id)
//         .map(|x| x.clone())
//         .map(Json)
// }

#[get("/person/{id}")]
async fn get_person_by_id(data: web::Data<PersonDB>, path: web::Path<PathInfo>) -> Option<Json<Person>>{
    data.people
        .get(&path.id)
        .map(|x| x.to_owned())
        .map(Json)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let person_db = web::Data::new(init_person_db());

    HttpServer::new(move || {
        App::new()
        .app_data(person_db.clone())
        .wrap(middleware::Logger::default())
        .service(get_person_by_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}



#[derive(Serialize, Deserialize, Clone)]
pub struct Person {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub age: u32,
}

#[derive( Deserialize)]
struct PathInfo{
    id: u32
}


fn init_person_db()-> PersonDB {
    let map = vec![
        Person{
            id: 0,
            last_name: "Peter".to_string(),
            first_name: "Lustig".to_string(),
            age: 30,
        },
        Person{
            id: 1,
            last_name: "Hans".to_string(),
            first_name: "Dampf".to_string(),
            age: 42,
        },
    ].into_iter()
    .map(|p| (p.id,p))
    .collect();
    PersonDB{
        people: map,
    }
}