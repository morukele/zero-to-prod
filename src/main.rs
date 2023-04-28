use std::net::TcpListener;

use zero2production::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8000").expect("Failed to connect to port 8000");

    run(listener)?.await
}
