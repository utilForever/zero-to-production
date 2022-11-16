use std::net::TcpListener;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    run(listener)?.await
}
