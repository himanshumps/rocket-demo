use actix_web::{get, web, App, Error, HttpResponse, HttpServer, Result};
use couchbase::*;
use std::env;
use std::io::BufReader;
use std::sync::Arc;

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
    let cb_cluster = Cluster::connect(
        env::var("COUCHBASE_STRING").unwrap(),
        env::var("COUCHBASE_USERNAME").unwrap(),
        env::var("COUCHBASE_PASSWORD}").unwrap(),
    );
    let arc_cluster = Arc::new(cb_cluster);
    let cb_bucket = arc_cluster.bucket(env::var("COUCHBASE_BUCKET").unwrap());
    let arc_bucket = Arc::new(cb_bucket);
    let cb_collection = arc_bucket.default_collection();
    let arc_collection = Arc::new(cb_collection);

    HttpServer::new(move || {
        App::new()
            .data(PaceCouchbase {
                collection: arc_collection.clone(),
            })
            .service(index)
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
