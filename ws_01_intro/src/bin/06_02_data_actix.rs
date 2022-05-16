use std::sync::{atomic::AtomicU32, Arc};

use actix_web::{get, web::{self, Json}, App, HttpServer,error, middleware, Responder,  Result as WebResult};
use serde::{Deserialize, Serialize};

pub struct Counter{
    count: AtomicU32
}

#[get("/counter/")]
async fn counter(data: web::Data<Counter>) -> String {
    data.count.fetch_add(1, std::sync::atomic::Ordering::SeqCst).to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let counter_data = web::Data::new(Counter{count: AtomicU32::new(0)});
    HttpServer::new(move || {
        App::new()
        .app_data(counter_data.clone())
        .wrap(middleware::Logger::default())
        .service(counter)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}