use clap::Parser;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::process;
use std::str::FromStr;

#[path = "lib.rs"]
mod enclave_server;

/// http server for handling attestation document requests
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// ip address of the server
    #[arg(short, long)]
    ip_addr: String,
}

async fn serve_attestion_doc(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(enclave_server::get_attestation_doc().into()))
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // We'll bind the server to X.X.X.X:X
    let addr = SocketAddr::from_str(&cli.ip_addr).unwrap_or_else(|e| {
        eprintln!("failed to create socket address: {}", e);
        process::exit(1);
    });

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
