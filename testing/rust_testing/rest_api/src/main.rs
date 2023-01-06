
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::Mutex;
use warp::{Filter, Rejection, http::StatusCode, reply, Reply};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Teststruct {
    pub name: String
}

#[tokio::main]
async fn main() {
    let root = warp::path::end().map(|| "Welcome to my warp server! LOOOOL");
    let testing = warp::path("testing")
    .map(|| "Welcome to my warp route! LOOOOL")
    .and(warp::post())
    .and_then(testhandler);
    let routes = root
                    .or(testing)
                    .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 5000)).await;
}

pub async fn testhandler(test: &str) -> Result<impl Reply, String> {
    print!("testhandler");

    Ok(reply::with_status(reply::json(&vec!(Teststruct {name: String::new()})), StatusCode::OK))
}