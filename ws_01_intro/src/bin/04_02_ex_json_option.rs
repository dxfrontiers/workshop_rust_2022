use actix_web::{post, get, web::{self, Json}, App, HttpServer, Result, Responder, middleware};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Greeting {
    firstname: String,
    lastname: String,
    include_len: Option<bool>
}


//curl -X POST -H "Content-Type: application/json" -d @json/greeting.json 127.0.0.1:8080/greeting
async fn greet_with_length_option(greeting: web::Json<Greeting>) -> String {
    let greeting = greeting.into_inner();
    if /*TODO*/greeting.include_len.unwrap_or(false) {
        let length = get_length_of_name(/*TODO*/&greeting);
        format!("Greetings, {} {}!, your name has {} chars.", greeting.firstname, greeting.lastname, length)
    }
    else{
        format!("Greetings, {} {}!", greeting.firstname, greeting.lastname)
    }
}

fn get_length_of_name(/*TODO*/greeting: &Greeting) -> usize{
    // greeting.firstname.chars().count() + greeting.lastname.chars().count()
    // greeting.firstname.len() + greeting.lastname.len()
    // TODO
    todo!()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
        .wrap(middleware::Logger::default())
        .service(web::resource("/greet").route(web::post().to(greet_with_length_option)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}





// ----------------- tests -------------------


#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_greet_bilbo() {
        test_greet(
            Greeting{ 
                firstname: "Bilbo".into(), 
                lastname: "Baginns".into(), 
                include_len: None 
            }
            , |res| assert_eq!(res,"Greetings, Bilbo Baginns!") 
        ).await;

        test_greet(
            Greeting{ 
                firstname: "Bilbo".into(), 
                lastname: "Baginns".into(), 
                include_len: Some(false)
            }
            , |res| assert_eq!(res,"Greetings, Bilbo Baginns!") 
        ).await;
    }

    #[actix_web::test]
    async fn test_greet_bilbo_len() {
        test_greet(
            Greeting{ 
                firstname: "Bilbo".into(), 
                lastname: "Baginns".into(), 
                include_len: Some(true) 
            }
            , |res| assert!(res.contains("12"))
        ).await;
    }
    async fn test_greet(greeting: Greeting, verifier: fn(String)){
        let resp = greet_with_length_option(web::Json(greeting)).await;
        println!("Response: {}", resp );
        verifier(resp)
    } 
}
