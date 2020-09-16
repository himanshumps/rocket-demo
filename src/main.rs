use couchbase::*;
use actix_web::{get, web, App, HttpResponse, HttpServer};

#[get("/getDetails/{id}")]
async fn index(web::Path(id): web::Path<String>, collection: web::Data<Collection>) -> HttpResponse {
    collection.get(id, GetOptions::default()).await {
        Ok(mut result) => Ok(HttpResponse::Ok().body(result),
        Err(e) => Ok(HttpResponse::InternalServerError().content_type("text/plain").body(e)))
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    // Use the default collection (needs to be used for all server 6.5 and earlier)
    let collection = Cluster::connect("couchbase://127.0.0.1", "Administrator", "password").bucket("travel-sample").default_collection();
   
    HttpServer::new(|| App::new().app_data(collection).service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}