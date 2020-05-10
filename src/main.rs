use tokio::prelude::*;
use rusty_chat::server::Server;
use rusty_chat::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let server = Server::new(8080);
    server.run().await
}
