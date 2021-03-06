use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use rust_schema_registry_server::handlers::{find_by_content_type, find_one, register};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        "rust_schema_registry_server=debug,actix_web=debug",
    );
    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .service(register)
            .service(find_by_content_type)
            .service(find_one)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
