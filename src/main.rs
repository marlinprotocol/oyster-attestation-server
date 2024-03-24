use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;
use std::str::FromStr;

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

    let addr = SocketAddr::from_str(&cli.ip_addr)?;

    let make_svc = make_service_fn(|_conn| {
        let service = service_fn(|_req| async {
            Ok::<_, Infallible>(Response::<Body>::new(
                oyster_attestation_server_ed25519::get_attestation_doc(pub_key).into(),
            ))
        });
        async move { Ok::<_, Infallible>(service) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    server.await?;
    Ok(())
}
