use std::env;

use actix_http::KeepAlive;
use actix_web::{App, get, HttpResponse, HttpServer, middleware::Logger, Result, web};
use couchbase::{Cluster, Collection, GetOptions};

/*use async_std::sync::Arc;*/
/*#[derive(Debug, Clone)]
struct PaceCouchbase (Collection);
*/
#[get("/getDetails/{id}")]
async fn index(
    web::Path(id): web::Path<String>,
    pace_couchbase: web::Data<Collection>,
) -> Result<HttpResponse, HttpResponse> {
    let results = match pace_couchbase
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
    let data = web::Data::new(Cluster::connect(
        env::var("COUCHBASE_STRING").unwrap(),
        env::var("COUCHBASE_USERNAME").unwrap(),
        env::var("COUCHBASE_PASSWORD").unwrap(),
    )
        .bucket(env::var("COUCHBASE_BUCKET").unwrap())
        .default_collection());
    /*
        let cb_cluster = Cluster::connect(
            env::var("COUCHBASE_STRING").unwrap(),
            env::var("COUCHBASE_USERNAME").unwrap(),
            env::var("COUCHBASE_PASSWORD").unwrap(),
        );
        let arc_cluster = Arc::new(cb_cluster);
        let cb_bucket = arc_cluster.bucket(env::var("COUCHBASE_BUCKET").unwrap());
        let arc_bucket = Arc::new(cb_bucket);
    */    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(Logger::new("\" % r\" %s %b \" % { Referer }i\" \"%{User-Agent}i\" %D"))
            .service(index)
    })
        .backlog(1024)
        .workers(num_cpus::get() * 4)
        .bind("0.0.0.0:8082")?
        .run()
        .await
}
