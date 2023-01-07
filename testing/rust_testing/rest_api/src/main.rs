use rand::Rng;
use std::net::SocketAddr;
use std::error::Error;
// use hyper::header::{Headers, AccessControlAllowOrigin};
use hyper::body::Buf;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{header, Body, Method, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use std::thread;


use tokio::net::{TcpListener, TcpStream};

use std::io;

async fn process(socket: TcpStream) {
    print!("Call from Tokio stuff\n");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // tokio::spawn(async move {
        // some work here
        // loop {
            // let (stream, _) = listener.accept().await?;
            
            // tokio::spawn(async move {
                // Http::new().serve_connection(stream, service_fn(cars_handler)).await;
                // if let Err(err) = Http::new().serve_connection(stream, service_fn(cars_handler)) {
                //     println!("Error serving connection: {:?}", err);
                // }
            // });
        //     print!("test");
        // }
    // });
    let mut listener = TcpListener::bind("127.0.0.1:3000").await?;
    // let mut headers = Headers::new();
    // headers.set(
    //     AccessControlAllowOrigin::Value(AccessControlAllowOrigin::Any.to_owned())
    // );
    print!("successfully opened port 3000 for localhost\n");
    
    tokio::spawn(async move {
        loop {
            let (stream, _) = match listener.accept().await {
                Ok(t) => t,
                Err(e) => panic!("Error")
            };
            // Process each socket concurrently.
            Http::new().serve_connection(stream, service_fn(cars_handler)).await;
            //process(stream).await
        }
    });
    
    loop {
        let now = Instant::now();
        while now.elapsed().as_millis() < 1000 {}
        print!("DMX BERECHNEN since: {}\n", now.elapsed().as_millis());
        
    }
    Ok(())
}

// async fn my_background_task(req: Request<Body>) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
//     loop {
//         let (stream, _) = listener.accept().await?;
        
//         tokio::spawn(async move {
//             Http::new().serve_connection(stream, service_fn(cars_handler)).await;
//             if let Err(err) = Http::new().serve_connection(stream, service_fn(cars_handler)) {
//                 println!("Error serving connection: {:?}", err);
//             }
//         });
//         print!("test");
//     }
// }

async fn cars_handler(req: Request<Body>) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
    print!("New Connection\n");
    let path = req.uri().path().to_owned();

    match req.uri().query() {
        Some(e) => e
            .to_owned()
            .split("&")
            .collect::<Vec<&str>>()
            .iter()
            .for_each(|e| {
                print!("{:?}\n", e.split("=").collect::<Vec<&str>>())
            }
        ),
        None => print!("No query parameters")
    };

    let path_segments = path.split("/").collect::<Vec<&str>>();
    let base_path = path_segments[1];

    // for segment in path_segments.iter() {
    //     print!("{}\n", segment);
    // }
    // Ok(Response::new(Body::from("Moin")));
    match (req.method(), base_path) {
        (&Method::GET, "cars") => {
            print!("Get detected");
            Ok(Response::new(Body::from("GET cars")))
        },

        (&Method::POST, "cars") => {
            print!("Post detected");
            // print!("requestbody: {:?}\n", req.body());
            return Ok(Response::new(Body::from("POST cars")))
        },

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}