use actix_web::{get, web, App, Error, HttpResponse, HttpServer, Result};
use couchbase::*;
use java_properties::read;
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
    let file_name = "couchbase.properties";

    let mut file = File::open(&file_name)?;
    let couchbase_map = read(BufReader::new(file))?;

    HttpServer::new(|| {
        App::new()
            .data(PaceCouchbase {
                collection: Cluster::connect(
                    couchbase_map.get(&"connection_string").unwrap(),
                    couchbase_map.get(&"username").unwrap(),
                    couchbase_map.get(&"password").unwrap(),
                )
                .bucket(couchbase_map.get(&"bucket").unwrap())
                .default_collection(),
            })
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}