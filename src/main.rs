use std::convert::Infallible;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;
use std::str::FromStr;

use clap::Parser;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Response, Server};

#[path = "lib.rs"]
mod enclave_server;

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

    let mut file = File::open(cli.pub_key)?;
    let mut pub_key = [0; 32];
    file.read_exact(&mut pub_key)?;
    println!("pub key: {:02x?}", pub_key);

    let addr = SocketAddr::from_str(&cli.ip_addr)?;

    let make_svc = make_service_fn(move |_conn| {
        let service = service_fn(move |_req| async move {
            Ok::<_, Infallible>(Response::<Body>::new(
                enclave_server::get_attestation_doc(pub_key).into(),
            ))
        });
        async move { Ok::<_, Infallible>(service) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    server.await?;
    Ok(())
}
