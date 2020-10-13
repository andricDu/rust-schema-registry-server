use actix_web::{App, HttpServer};
use rust_schema_registry_server::handlers::register;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(register)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
