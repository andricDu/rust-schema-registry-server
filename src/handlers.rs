use actix_web::{post, web, Result, HttpResponse, Error};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::models::{Schema, NewSchema};
use crate::schema::schemas;

// Who wants to type this out every time???
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[post("/")]
pub async fn register(
    pool: web::Data<DbPool>,
    new_schema: web::Json<NewSchema>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let ret_schema = web::block(move || 
        diesel::insert_into(schemas::table).values(&*new_schema).execute(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    println!("schema: {:?}", &ret_schema);
    Ok(HttpResponse::Ok().json(ret_schema))
}


//@RequestMapping(method = RequestMethod.GET, produces = "application/json", path = "/{subject}/{format}/v{version}")
//@RequestMapping(method = RequestMethod.GET, produces = "application/json", path = "/schemas/{id}")
//@GetMapping(produces = APPLICATION_JSON_VALUE, path = "/{subject}/{format}")
//@RequestMapping(value = "/{subject}/{format}/v{version}", method = RequestMethod.DELETE)
//@RequestMapping(value = "/schemas/{id}", method = RequestMethod.DELETE)
//@RequestMapping(value = "/{subject}", method = RequestMethod.DELETE)
