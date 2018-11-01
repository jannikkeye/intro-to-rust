extern crate hyper;

use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use std::fs::File;
use std::io::prelude::*;

fn write_count(count: u8) -> std::io::Result<()> {
    let mut file = File::create("count.txt").unwrap();
    write!(file, "{}", count).unwrap();
    Ok(())
}

fn read_count() -> std::io::Result<u8> {
    let mut file = File::open("count.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    Ok(contents.parse::<u8>().unwrap())
}

fn hello_world(_req: Request<Body>) -> Response<Body> {
    let mut count = read_count().unwrap();

    count = count + 1;
    write_count(count).unwrap();
    Response::new(Body::from(format!("Service called {} times.", count)))
}

fn main() {
    write_count(0).unwrap();
    let addr = ([127, 0, 0, 1], 3000).into();

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let new_svc = || {
        // service_fn_ok converts our function into a `Service`
        service_fn_ok(hello_world)
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    // Run this server for... forever!
    hyper::rt::run(server);
}
