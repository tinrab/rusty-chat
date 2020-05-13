use futures::stream::{StreamExt, TryStreamExt};
use futures::TryFutureExt;
use log::{error, info};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

use crate::client::Client;
use crate::error::{Error, Result};
use crate::hub::Hub;
use crate::proto::{Input, InputParcel};

pub struct Server {
    port: u16,
    hub: Hub,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Server {
            port,
            hub: Hub::new(),
        }
    }

    pub async fn run(&self) -> Result<()> {
        let (input_sender, input_receiver) = mpsc::unbounded_channel::<InputParcel>();

        let hub = self.hub.run(input_receiver);
        let listening = self.listen(input_sender);

        tokio::try_join!(listening, hub).map(|result| result.0)
    }

    async fn listen(&self, input_sender: UnboundedSender<InputParcel>) -> Result<()> {
        let mut listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).await?;
        info!("Running on port {}...", self.port);

        while let Ok((stream, _)) = listener.accept().await {
            if let Err(err) = self.handle_connection(stream, input_sender.clone()).await {
                error!("Failed to handle connection: {}", err);
            }
        }

        Ok(())
    }

    async fn handle_connection(
        &self,
        stream: TcpStream,
        input_sender: UnboundedSender<InputParcel>,
    ) -> Result<()> {
        let ws = tokio_tungstenite::accept_async(stream).await?;
        let output_receiver = self.hub.subscribe();

        tokio::spawn(async move {
            let (ws_sink, ws_stream) = ws.split();
            let client = Client::new();

            info!("Client {} connected", client.id);

            let reading = client
                .read_input(ws_stream)
                .try_for_each(|input_parcel| async {
                    input_sender.send(input_parcel).unwrap();
                    Ok(())
                });

            let writing = client
                .write_output(output_receiver.into_stream())
                .forward(ws_sink)
                .map_err(Error::WebSocket);

            if let Err(err) = tokio::select! {
                result = reading => result,
                result = writing => result,
            } {
                error!("Client connection error: {}", err);
            }

            info!("Client {} disconnected", client.id);
        });

        Ok(())
    }
}
