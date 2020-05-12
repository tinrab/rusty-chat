use std::{error, result};

use futures::stream::SplitStream;
use futures::{future, Stream, StreamExt, TryStream, TryStreamExt};
use log::error;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite, WebSocketStream};
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::proto::{Input, InputMessage, OutputMessage, SendInput};

#[derive(Copy)]
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
    ) -> impl Stream<Item = Result<InputMessage>> {
        let client_id = self.id;
        stream
            .take_while(|message| {
                future::ready(if let Ok(message) = message {
                    message.is_text()
                } else {
                    false
                })
            })
            .map(move |message| match message {
                Err(err) => Err(Error::WebSocket(err)),
                Ok(message) => {
                    let data = message.to_text().unwrap().trim();
                    Ok(InputMessage::new(
                        client_id,
                        Input::Send(SendInput::new(data)),
                    ))
                }
            })
    }

    pub fn write_output<S, E>(
        &self,
        stream: S,
    ) -> impl Stream<Item = tungstenite::Result<tungstenite::Message>>
    where
        S: TryStream<Ok = OutputMessage, Error = E>
            + Stream<Item = result::Result<OutputMessage, E>>,
        E: error::Error,
    {
        let client_id = self.id;
        stream
            .try_filter(move |output_message| future::ready(output_message.is_target(client_id)))
            .map_ok(|output_message| {
                let data = serde_json::to_string(output_message.output()).unwrap();
                tungstenite::Message::text(data)
            })
            .map_err(|err| {
                error!("Client write error: {}", err);
                tungstenite::Error::Http(tungstenite::http::StatusCode::INTERNAL_SERVER_ERROR)
            })
    }
}
