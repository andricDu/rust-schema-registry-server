use actix_web::{get, post, web, Error, HttpResponse, Result};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use log::debug;

use crate::models::{NewSchema, NewSchemaRequest, Schema};
use crate::schema::schemas;
use crate::schema::schemas::dsl::*;
use crate::validator::avro_validator;

// Who wants to type this out every time???
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
type DbConn = PooledConnection<ConnectionManager<PgConnection>>;

#[post("/")]
pub async fn register(
    pool: web::Data<DbPool>,
    new_schema: web::Json<NewSchemaRequest>,
) -> Result<HttpResponse, Error> {
    // Short Return on Bad Format
    if new_schema.format != "avro" {
        return Ok(HttpResponse::BadRequest().finish());
    }

    if !avro_validator::is_valid(&new_schema.definition) {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let ret_schema = web::block(move || save_new_schema(pool, new_schema.to_owned()))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    println!("schema: {:?}", &ret_schema);
    Ok(HttpResponse::Ok().json(ret_schema))
}

#[get("/{subject}/{format}/v{version}")]
pub async fn find_by_content_type(
    pool: web::Data<DbPool>,
    path_params: web::Path<(String, String, i32)>,
) -> Result<HttpResponse, Error> {
    let path = path_params.into_inner();
    let sub = path.0;
    let fmt = path.1;
    let ver = path.2;

    let conn = pool.get().expect("couldn't get db connection from pool");

    let ret_schema = web::block(move || find_schema(&conn, sub, fmt, ver))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(ret_schema))
}

#[get("/schemas/{id}")]
pub async fn find_one(
    pool: web::Data<DbPool>,
    path_params: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let key = path_params.0;

    let conn = pool.get().expect("couldn't get db connection from pool");
    let ret_schema = web::block(move || find_by_id(&conn, key))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(ret_schema))
}

//@RequestMapping(method = RequestMethod.GET, produces = "application/json", path = "/schemas/{id}")
//@GetMapping(produces = APPLICATION_JSON_VALUE, path = "/{subject}/{format}")
//@RequestMapping(value = "/{subject}/{format}/v{version}", method = RequestMethod.DELETE)
//@RequestMapping(value = "/schemas/{id}", method = RequestMethod.DELETE)
//@RequestMapping(value = "/{subject}", method = RequestMethod.DELETE)

fn save_new_schema(
    pool: web::Data<DbPool>,
    new_schema_request: NewSchemaRequest,
) -> Result<Schema, diesel::result::Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let matching_schemas = get_by_subject_and_format_ordered(
        &conn,
        &new_schema_request.subject,
        &new_schema_request.format,
    );

    let res: Schema;

    if matching_schemas.len() == 0 {
        let new_schema = NewSchema {
            version: 1,
            format: new_schema_request.format,
            subject: new_schema_request.subject,
            definition: new_schema_request.definition,
        };
        res = diesel::insert_into(schemas::table)
            .values(new_schema)
            .get_result(&conn)?;
    } else {
        res = match avro_validator::get_matching_schema_from_def(
            matching_schemas.iter().collect(),
            &new_schema_request.definition,
        ) {
            Some(s) => s.clone(),
            None => {
                debug!("This is a new schema: {0}", new_schema_request.subject);
                let new_schema = NewSchema {
                    version: matching_schemas.last().unwrap().version + 1,
                    format: new_schema_request.format,
                    subject: new_schema_request.subject,
                    definition: new_schema_request.definition,
                };
                diesel::insert_into(schemas::table)
                    .values(new_schema)
                    .get_result(&conn)?
            }
        };
    }

    Ok(res)
}

fn get_by_subject_and_format_ordered(conn: &DbConn, sub: &String, fmt: &String) -> Vec<Schema> {
    schemas
        .filter(subject.eq(sub))
        .filter(format.eq(fmt))
        .order(version.desc())
        .limit(1)
        .load::<Schema>(conn)
        .unwrap()
}

fn find_schema(
    conn: &DbConn,
    sub: String,
    fmt: String,
    ver: i32,
) -> Result<Schema, diesel::result::Error> {
    schemas
        .filter(subject.eq(sub))
        .filter(format.eq(fmt))
        .filter(version.eq(ver))
        .limit(1)
        .first::<Schema>(conn)
}

fn find_by_id(conn: &DbConn, key: i32) -> Result<Schema, diesel::result::Error> {
    schemas.find(key).first::<Schema>(conn)
}
