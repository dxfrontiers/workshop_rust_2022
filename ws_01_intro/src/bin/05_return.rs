use actix_web::{post, get, web::{self, Json}, App, HttpServer, error, Result as WebResult, Responder, middleware};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Person {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub age: u32,
}

#[get("/v1/person/bond")]
async fn get_person_by_id_01() -> Json<Person> {
    let person = Person{
        id: 007,
        last_name: "Bond".to_string(),
        first_name: "James".to_string(),
        age: 30,
    };
    Json(person)
}
#[derive( Deserialize)]
struct PathInfo{
    id: u32
}

#[get("/v2/person/{id}")]
async fn get_person_by_id_02( path: web::Path<PathInfo>) -> Json<Person> {
    // let id = path.into_inner();   

    let person = Person{
        id: path.id,
        last_name: "John".to_string(),
        first_name: "Doe".to_string(),
        age: 30,
    };
    Json(person)
}

#[get("/v3/person/{id}")]
async fn get_person_by_id_03(path: web::Path<PathInfo>) -> Json<Person> { 
    let person = people().remove(path.id as usize);
    Json(person)
}

#[get("/v4/person/{id}")]
async fn get_person_by_id_04(path: web::Path<PathInfo>) -> Option<Json<Person>> {
    people()
        .into_iter() // bad
        .filter(|p|p.id==path.id)
        .next()
        .map(Json)
}

#[get("/v5/person/{id}")]
async fn get_person_by_id_05(path: web::Path<PathInfo>) -> Option<Json<Person>> {
    people()
        .iter()
        .filter(|p|p.id==path.id)
        .next() //Option<&Person>
        .map(|p| p.clone()) //Option<Person>
        .map(Json)
}

// bonus
#[get("/v6/person/{id}")]
async fn get_person_by_id_06(path: web::Path<PathInfo>) -> WebResult<impl Responder> {
    people()
        .iter()
        .filter(|p|p.id==path.id)
        .next()
        .ok_or(error::ErrorNotFound(format!("Could not find person with id {}",path.id)))
        .map(serde_json::to_string)        
}


// TODO: aufgabe
#[get("/person/")]
async fn get_persons() -> Json<Vec<Person>> {
    Json(people())
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
        .wrap(middleware::Logger::default())
        .service(get_person_by_id_01)
        .service(get_person_by_id_02)
        .service(get_person_by_id_03)
        .service(get_person_by_id_04)
        .service(get_person_by_id_05)
        .service(get_person_by_id_06)
        .service(get_persons)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// TODO Erklärung
fn people()-> Vec<Person> {
    vec![
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
    ]
} 