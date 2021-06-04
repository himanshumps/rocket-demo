use actix_web::{get, middleware::Logger, web, App, Error, HttpResponse, HttpServer, Result};
use couchbase::{Bucket, Cluster, GetOptions};
use std::env;
use std::sync::Arc;

#[get("/getDetails/{id}")]
async fn index(
    web::Path(id): web::Path<String>,
    bucket: web::Data<Arc<Bucket>>,
) -> Result<HttpResponse, Error> {
    let results = bucket
        .as_ref()
        .default_collection()
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
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let cb_cluster = Cluster::connect(
        env::var("COUCHBASE_STRING").unwrap(),
        env::var("COUCHBASE_USERNAME").unwrap(),
        env::var("COUCHBASE_PASSWORD").unwrap(),
    );
    let arc_cluster = Arc::new(cb_cluster);
    let cb_bucket = arc_cluster.bucket(env::var("COUCHBASE_BUCKET").unwrap());
    let arc_bucket = Arc::new(cb_bucket);
    HttpServer::new(move || {
        App::new()
            .data(arc_bucket.clone())
            .wrap(Logger::default())
            .service(index)
    })
    .bind("0.0.0.0:8082")?
    .run()
    .await
}
