use actix_web::{get, web, App, Error, HttpResponse, HttpServer, Result};
use couchbase::*;
use java_properties::read;
use std::env;
use std::fs::File;
use std::io::BufReader;

pub struct PaceCouchbase {
    collection: Collection,
}
#[get("/getDetails/{id}")]
async fn index(
    web::Path(id): web::Path<String>,
    pace_couchbase: web::Data<PaceCouchbase>,
) -> Result<HttpResponse, Error> {
    let results = match pace_couchbase
        .collection
        .get(id, GetOptions::default())
        .await
    {
        Ok(r) => HttpResponse::Ok().body(format!("{:?}", r)),
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    };

    Ok(results)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(move || {
        let file_name = "couchbase.properties";
        let file = File::open(&file_name).unwrap();
        let couchbase_map = read(BufReader::new(file)).unwrap();
        let couchbase_default_collection = Cluster::connect(
            env::var("COUCHBASE_STRING").unwrap(),
            env::var("COUCHBASE_USERNAME").unwrap(),
            env::var("COUCHBASE_PASSWORD}").unwrap(),
        )
        .bucket(env::var("COUCHBASE_BUCKET").unwrap())
        .default_collection();
        App::new()
            .data(PaceCouchbase {
                collection: couchbase_default_collection,
            })
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
