
use actix_web::{post, get, web::{self, Json}, App, HttpServer, Error as WebError, Result, Responder, middleware, HttpResponse};

use r2d2_sqlite::SqliteConnectionManager;
use serde::{Deserialize, Serialize};

mod model;
mod db;

use crate::model::{Person, NewPerson};
use db::{Pool};


#[derive(Deserialize)]

struct PersonQuery{
    age: u32
}

#[get("/person/{user_id}")]
async fn get_person_by_id(db: web::Data<Pool>, path: web::Path<u32>) -> Result<Option<Json<Person>>, WebError>{
    let id = path.into_inner();    
    let result = db::get_person_by_id(&db, id)
        .await?
        .map(|p| web::Json(p));
    Ok(result)
}

#[post("/person")]
async fn post_person(db: web::Data<Pool>, person: web::Json<NewPerson>) ->  Result<Json<Person>, WebError> {    
    db::insert_person(&db, person.into_inner())
        .await
        .map(|p| web::Json(p))
}

#[get("/person")]
async fn get_person_query(db: web::Data<Pool>, query: web::Query<PersonQuery>) ->  Result<Json<Vec<Person>>, WebError> {    
    db::get_person_by_age(&db, query.age)
        .await
        .map(|p| web::Json(p))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let manager = SqliteConnectionManager::file("people.db");
    let pool = Pool::new(manager).unwrap();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .wrap(middleware::Logger::default())
        .service(get_person_by_id)
        .service(get_person_query)
        .service(post_person)
    })
    .bind(("127.0.0.1", 8080))?
    // .workers(2)
    .run()
    .await
}