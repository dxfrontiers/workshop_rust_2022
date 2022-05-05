use actix_web::{ rt, App, HttpServer, middleware, web};

// curl 127.0.0.1:8080/hello

async fn hello() -> String {
    String::from("Hello world")
}


fn main() -> std::io::Result<()> {
    rt::System::new().block_on(
        HttpServer::new(move || {
            App::new()
            .wrap(middleware::Logger::default())
            .route("/hello", web::get().to(hello))
        })
        .bind(("127.0.0.1", 8080))?
        .run()
    )
}