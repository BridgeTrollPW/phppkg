use actix_web::{get, App, HttpServer, Responder};
use routes::package::{self, package_list};
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(package_list))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
