use rusty_chat::error::Result;
use rusty_chat::server::Server;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let server = Server::new(8080);
    server.run().await
}
