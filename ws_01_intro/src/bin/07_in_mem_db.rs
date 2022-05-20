use std::collections::HashMap;

use actix_web::{get,post, web::{self, Json}, App, HttpServer, middleware, delete, HttpResponse};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;


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

struct PersonDB{
    store: Mutex<HashMap<u32,Person>>
}

impl PersonDB{
    async fn insert(&self, person: Person) -> /* TODO */ {
        todo!("insert person in db")
    }
    async fn get(&self, id: u32) -> /* TODO */ {
        todo!("get one person from db")
    }
    async fn list(&self) -> /* TODO */{
        todo!("list all person from db")
    }
    async fn delete(&self, id: u32) -> /* TODO */{
        todo!("delete person from db")
    }
}

#[get("/person/{id}")]
async fn get_person_by_id(data: web::Data<PersonDB>, path: web::Path<PathInfo>) -> Option<Json<Person>> {
    todo!("insert person")
}

#[get("/person/")]
async fn list_person(data: web::Data<PersonDB> ) -> Json<Vec<Person>> {
    todo!("get one person")
}

#[post("/person/")]
async fn post_person(data: web::Data<PersonDB>, person: web::Json<Person>) -> HttpResponse {
    todo!("list all person")
}

#[delete("/person/{id}")]
async fn delete_person(data: web::Data<PersonDB>, path: web::Path<PathInfo>) -> Option<Json<Person>> {
    todo!("delete person")
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
        .service(list_person)
        .service(post_person)
        .service(delete_person)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
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
        store: Mutex::new(map),
    }
}