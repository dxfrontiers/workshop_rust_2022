use actix_web::{post, get, web::{self, Json}, App, HttpServer, Result, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Greeting {
    name: String,
    include_len: Option<bool>
}

#[derive(Serialize)]
struct Person {
    id: u32,
    name: String,
    age: u32,
    motto: String
}

// curl 127.0.0.1:8080/hello
#[get("/hello_str")]
async fn hello_str() -> &'static str {
    "Hello there"
}

// curl 127.0.0.1:8080/hello
#[get("/hello")]
async fn hello() -> String {
    String::from("Hello there")
}

// curl -v "127.0.0.1:8080/greeting?name=GeneralKenobi"
#[get("/greeting")]
async fn greet_query(greeting: web::Query<Greeting>) -> String {
    format!("Hello There  {}!", greeting.name)
}

// curl -X POST -H 'Content-Type: application/json' -d '{"name":"test123"}' 127.0.0.1:8080/greeting
// #[post("/greeting")]
// async fn greet(greeting: web::Json<Greeting>) -> String {
//     format!("Greetings, {}!", greeting.name)
// }

// curl -X POST -H 'Content-Type: application/json' -d '{"name":"test123","include_len":true}' 127.0.0.1:8080/greeting
#[post("/greeting")]
async fn greet(greeting: web::Json<Greeting>) -> String {
    let greeting = greeting.into_inner();
    if greeting.include_len.unwrap_or(false) {
        let length = get_length_of_name(greeting.name.clone());
        format!("Greetings, {}!, your name has {} chars.", greeting.name, length)
    }
    else{
        format!("Greetings, {}!", greeting.name)
    }
}

fn get_length_of_name(name: String) -> usize{
    name.len()
}


#[get("/person/{user_id}")]
async fn get_person(path: web::Path<u8>) -> impl Responder {
    Json(Person{
        id: path.into_inner() as u32,
        name: "Bilbo Baggins".to_string(),
        age: 33,
        motto: "Lol 123".to_string()
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(hello)
        .service(hello_str)
        .service(greet_query)
        .service(greet)
        .service(get_person)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}