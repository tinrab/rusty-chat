use crate::error::{Error, Result};
use crate::proto::{Input, Output, SendInput, InputMessage};
use futures::stream::SplitStream;
use futures::{future, FutureExt, Stream, StreamExt, TryFutureExt, TryStreamExt};
use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tokio_tungstenite::{tungstenite, WebSocketStream};
use uuid::Uuid;

pub struct Client {
    id: Uuid,
}

impl Client {
    pub fn new() -> Self {
        Client { id: Uuid::new_v4() }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn read_input(
        &self,
        stream: SplitStream<WebSocketStream<TcpStream>>,
    ) -> impl Stream<Item=Result<InputMessage>> {
        let client_id = self.id.clone();
        stream
            .take_while(|message| {
                future::ready(if let Ok(message) = message {
                    message.is_text()
                } else {
                    false
                })
            })
            .map(move |message| {
                match message {
                    Err(err) => Err(Error::WebSocket(err)),
                    Ok(message) => {
                        let data = message.to_text().unwrap().trim();
                        Ok(InputMessage::new(client_id, Input::Send(SendInput::new(data))))
                    }
                }
            })
    }
}
