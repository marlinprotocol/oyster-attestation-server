use std::error::Error;

use axum::{http::Method, routing::get, Router};
use tower_http::cors::{Any,CorsLayer};
use clap::Parser;

/// http server for handling attestation document requests
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// ip address of the server (e.g. 127.0.0.1:1300)
    #[arg(short, long)]
    ip_addr: String,

    /// path to public key file (e.g. /app/id.pub)
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

    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any);

    let app = Router::new()
        .route(
            "/attestation/raw",
            get(|| async { oyster_attestation_server::get_attestation_doc(pub_key) }),
        )
        .route(
            "/attestation/hex",
            get(|| async { oyster_attestation_server::get_hex_attestation_doc(pub_key) }),
        ).layer(cors);
    let listener = tokio::net::TcpListener::bind(&cli.ip_addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
