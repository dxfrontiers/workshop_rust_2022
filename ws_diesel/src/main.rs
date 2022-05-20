
#[macro_use]
extern crate diesel;

use actix_web::{post, get, web::{self, Json}, error, App, HttpServer, Error as WebError, Result,  middleware};

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod model;
mod schema;
mod db;

use crate::model::{Person, NewPerson};
use crate::db::DbPool;

#[get("/person/{user_id}")]
async fn get_person_by_id(db: web::Data<DbPool>, path: web::Path<i32>) -> Result<Option<Json<Person>>, WebError>{
    let id = path.into_inner();   
    db::get_person_by_id(&db, id)
        .await
        .map(|res| res.map(Json))
        .map_err(error::ErrorInternalServerError)
}
#[get("/person")]
async fn get_person_all(db: web::Data<DbPool>) -> Result<Json<Vec<Person>>, WebError>{
    let result = db::get_person_all(&db)
        .await        
        .map_err(error::ErrorInternalServerError)?
        ;
    Ok(web::Json(result))
}

#[post("/person")]
async fn post_person(db: web::Data<DbPool>, person: web::Json<NewPerson>) ->  Result<Json<Person>, WebError> {    
    db::insert_person(&db, person.into_inner())
        .await
        .map(web::Json)
        .map_err(error::ErrorInternalServerError)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let manager = ConnectionManager::<SqliteConnection>::new("person.db");
    let pool = r2d2::Pool::new(manager).unwrap();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .wrap(middleware::Logger::default())
        .service(post_person)
        .service(get_person_by_id)
        .service(get_person_all)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

