use std::net::TcpListener;

use emailapi::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listner = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind 8000 port");
    run(listner).await?.await
}
