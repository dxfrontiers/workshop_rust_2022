use actix_web::{ get, App, HttpServer, middleware};

// curl 127.0.0.1:8080/hello
#[get("/hello")]
async fn hello() -> String {
    String::from("Hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .wrap(middleware::Logger::default())
        .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}