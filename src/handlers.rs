use actix_web::{post, web, HttpResponse};
use crate::models::Schema;

#[post("/")]
pub async fn register(schema: web::Json<Schema>) -> HttpResponse {
    println!("schema: {:?}", &schema);
    HttpResponse::Ok().json(schema.0)
}


//@RequestMapping(method = RequestMethod.GET, produces = "application/json", path = "/{subject}/{format}/v{version}")
//@RequestMapping(method = RequestMethod.GET, produces = "application/json", path = "/schemas/{id}")
//@GetMapping(produces = APPLICATION_JSON_VALUE, path = "/{subject}/{format}")
//@RequestMapping(value = "/{subject}/{format}/v{version}", method = RequestMethod.DELETE)
//@RequestMapping(value = "/schemas/{id}", method = RequestMethod.DELETE)
//@RequestMapping(value = "/{subject}", method = RequestMethod.DELETE)
