use actix_web::{get, web, App, Error, HttpResponse, HttpServer, Result};
use couchbase::*;
use java_properties::read;
use std::fs::File;
use std::io::BufReader;
//use std::sync::Arc;

#[get("/getDetails/{id}")]
async fn index(
    web::Path(id): web::Path<String>,
    pace_bucket: web::Data<Bucket>,
) -> Result<HttpResponse, Error> {
    let results = match pace_bucket
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
    env_logger::init();

    HttpServer::new(move || {
        let file_name = "couchbase.properties";
        let file = File::open(&file_name).unwrap();
        let couchbase_map = read(BufReader::new(file)).unwrap();
        let tmp_bucket = Cluster::connect(
            couchbase_map
                .get::<str>(&"connection_string".to_string())
                .unwrap(),
            couchbase_map.get::<str>(&"username".to_string()).unwrap(),
            couchbase_map.get::<str>(&"password".to_string()).unwrap(),
        )
        .bucket(couchbase_map.get::<str>(&"bucket".to_string()).unwrap())
        .unwrap();
        //let arc_bucket = Arc::new(tmp_bucket);
        App::new().data(tmp_bucket.clone()).service(index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
