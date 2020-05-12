use log::error;

use rusty_chat::server::Server;

#[tokio::main]
async fn main() {
    env_logger::init();

    let server = Server::new(8080);
    if let Err(err) = server.run().await {
        error!("{}", err);
    }
}
