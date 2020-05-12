use std::any::Any;

use futures::stream::{StreamExt, TryStreamExt};
use futures::{future, SinkExt, TryFutureExt};
use log::{error, info};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::{broadcast, mpsc};
use tokio::time;

use crate::client::Client;
use crate::error::{Error, Result};
use crate::proto::{InputMessage, Output, OutputMessage};

const ALIVE_INTERVAL: time::Duration = time::Duration::from_secs(1);
const OUTPUT_CHANNEL_SIZE: usize = 3;

pub struct Server {
    port: u16,
    output_sender: broadcast::Sender<OutputMessage>,
}

impl Server {
    pub fn new(port: u16) -> Self {
        let (output_sender, _) = broadcast::channel(OUTPUT_CHANNEL_SIZE);
        Server {
            port,
            output_sender,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let listening = self.listen();
        let ticking_alive = self.tick_alive();
        tokio::try_join!(listening, ticking_alive).map(|result| result.0)
    }

    async fn listen(&self) -> Result<()> {
        let mut listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).await?;
        info!("Running on port {}...", self.port);

        let (input_sender, mut input_receiver) = mpsc::unbounded_channel();
        let processing_input = self.process_input(&mut input_receiver);

        let listening = async {
            while let Ok((stream, _)) = listener.accept().await {
                if let Err(err) = self.handle_connection(stream, input_sender.clone()).await {
                    error!("Failed to handle connection: {}", err);
                }
            }
        };

        tokio::join!(listening, processing_input);
        Ok(())
    }

    async fn tick_alive(&self) -> Result<()> {
        let mut interval = time::interval(ALIVE_INTERVAL);
        loop {
            interval.tick().await;
            if self.output_sender.receiver_count() > 0 {
                self.output_sender
                    .send(OutputMessage::new(Output::Alive))
                    .unwrap();
            }
        }
    }

    async fn process_input(&self, input_receiver: &mut UnboundedReceiver<InputMessage>) {
        while let Some(input_message) = input_receiver.recv().await {
            println!("input: {:?}", input_message);
        }
    }

    async fn handle_connection(
        &self,
        stream: TcpStream,
        input_sender: UnboundedSender<InputMessage>,
    ) -> Result<()> {
        let ws = tokio_tungstenite::accept_async(stream).await?;
        let output_stream = self.output_sender.subscribe();

        tokio::spawn(async move {
            let (mut ws_sink, ws_stream) = ws.split();
            let client = Client::new();
            info!("Client {} connected", client.id());

            let reading = client
                .read_input(ws_stream)
                .try_for_each(|input_message| async {
                    input_sender
                        .send(input_message)
                        .map_err(|err| Error::System(err.to_string()))
                });

            let writing = client
                .write_output(output_stream.into_stream())
                .forward(ws_sink)
                .map_err(|err| Error::WebSocket(err));

            if let Err(err) = tokio::select! {
                result = reading => result,
                result = writing => result,
            } {
                error!("Client connection error: {}", err);
            }

            info!("Client {} disconnected", client.id());
        });

        Ok(())
    }
}
