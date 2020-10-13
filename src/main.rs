use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use rust_schema_registry_server::model::dto::Schema;


#[post("/")]
async fn register(schema: web::Json<Schema>) -> HttpResponse {
    println!("schema: {:?}", &schema);
    HttpResponse::Ok().json(schema.0)
}


//@RequestMapping(method = RequestMethod.GET, produces = "application/json", path = "/{subject}/{format}/v{version}")
//@RequestMapping(method = RequestMethod.GET, produces = "application/json", path = "/schemas/{id}")
//@GetMapping(produces = APPLICATION_JSON_VALUE, path = "/{subject}/{format}")
//@RequestMapping(value = "/{subject}/{format}/v{version}", method = RequestMethod.DELETE)
//@RequestMapping(value = "/schemas/{id}", method = RequestMethod.DELETE)
//@RequestMapping(value = "/{subject}", method = RequestMethod.DELETE)


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(register)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}