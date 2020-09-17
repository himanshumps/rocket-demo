use actix_web::{get, web, App, HttpResponse, HttpServer};
use couchbase::*;

pub struct PaceCouchbase {
    collection: Collection,
}
#[get("/getDetails/{id}")]
async fn index(
    web::Path(id): web::Path<String>,
    paceCouchbase: web::Data<PaceCouchbase>,
) -> Result<HttpResponse, Error> {
    let results = match paceCouchbase
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
    // Use the default collection (needs to be used for all server 6.5 and earlier)

    HttpServer::new(|| {
        App::new()
            .data(PaceCouchbase {
                collection: Cluster::connect("couchbase://127.0.0.1", "Administrator", "password")
                    .bucket("travel-sample")
                    .default_collection(),
            })
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
