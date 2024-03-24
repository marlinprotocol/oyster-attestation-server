use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;
use std::str::FromStr;

use axum::{routing::get, Router};
use clap::Parser;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Response, Server};

/// http server for handling attestation document requests
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// ip address of the server
    #[arg(short, long)]
    ip_addr: String,

    /// path to public key file
    #[arg(short, long)]
    pub_key: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // leak in order to get a static slice
    // okay to do since it will get cleaned up on exit
    let pub_key = std::fs::read(cli.pub_key)?.leak::<'static>();
    println!("pub key: {:02x?}", pub_key);

    let app = Router::new().route(
        "/attestation/raw",
        get(|| async { oyster_attestation_server::get_attestation_doc(pub_key) }),
    );
    let listener = tokio::net::TcpListener::bind(&cli.ip_addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
