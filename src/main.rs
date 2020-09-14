#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use couchbase::{Cluster, QueryOptions};
use futures::executor::block_on;
use futures::stream::StreamExt;
use rocket::State;
use rocket_contrib::json::Json;
use serde_json::Value;

struct Couchbase {
    cluster: Cluster,
}

#[get("/airlines")]
fn index(db: State<Couchbase>) -> Json<Vec<Value>> {
    let mut result = block_on(db.cluster.query(
        "select `travel-sample`.* from `travel-sample` where type = 'airline' limit 10",
        QueryOptions::default(),
    ))
    .expect("Do Something with Error");

    let airlines = result.rows().map(|r| r.unwrap()).collect::<Vec<Value>>();
    Json(block_on(airlines))
}

fn main() {
    rocket::ignite()
        .manage(Couchbase {
            cluster: Cluster::connect("127.0.0.1", "Administrator", "password"),
        })
        .mount("/", routes![index])
        .launch();
}
