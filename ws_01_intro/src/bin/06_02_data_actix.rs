use actix_web::{get, web::{self, Json}, App, HttpServer,error, middleware, Responder,  Result as WebResult};
use serde::{Deserialize, Serialize};

pub struct PersonDB{
    people: Vec<Person>
}

// VORSICHT
#[get("/person/{user_id}")]
async fn get_person_by_id(data: web::Data<PersonDB>, path: web::Path<PathInfo>) -> WebResult<impl Responder> {
    // data.people.push(sample_person());
    data.people // VORSICHT
        .iter()
        .filter(|p|p.id==path.id)
        .next()
        .ok_or(error::ErrorNotFound(format!("Could not find person with id {}",path.id)))
        .map(serde_json::to_string)  
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(init_person_db()))
        .wrap(middleware::Logger::default())
        .service(get_person_by_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}



#[derive(Serialize, Deserialize)]
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


fn init_person_db()-> Vec<Person> {
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

fn sample_person () -> Person{
    Person{
        id: 1,
        last_name: "Sample".to_string(),
        first_name: "Person".to_string(),
        age: 42,
    }
}