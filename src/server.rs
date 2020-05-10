use crate::error::{Error, Result};
use tokio::net::{TcpListener, TcpStream};
use tokio::time;
use futures::StreamExt;

const ALIVE_INTERVAL: time::Duration = time::Duration::from_secs(1);

pub struct Server {
    port: u16,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Server { port }
    }

    pub async fn run(&self) -> Result<()> {
        let listening = self.listen();
        let ticking_alive = self.tick_alive();

        // future::join(listening, ticking_alive).await;
        // futures::join!(listening, ticking_alive);
        tokio::try_join!(listening, ticking_alive);
        Ok(())
    }

    async fn listen(&self) -> Result<()> {
        // let mut listener = TcpListener::bind(format!("127.0.0.1:{}", self.port))
        //     .await
        //     .map_err(|e| Error::System(e.to_string()))?;
        let mut listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).await?;

        while let Ok((stream, _)) = listener.accept().await {
            println!("accepted: {:?}", stream);
        }

        Ok(())
    }

    async fn tick_alive(&self) -> Result<()> {
        let mut interval = time::interval(ALIVE_INTERVAL);
        loop {
            interval.tick().await;
            println!("TICK");
        }
    }
}
