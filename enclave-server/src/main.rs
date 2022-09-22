use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

#[path="lib.rs"]
mod enclave_server;


async fn serve_attestion_doc(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(enclave_server::get_attestation_doc().into()))
}


#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:32
    let addr = SocketAddr::from(([127, 0, 0, 1], 32));

    // A `Service` is needed for every connection, so this
    // creates one from our `serve_attestion_doc` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(serve_attestion_doc))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}