use std::pin::Pin;
use std::{error, fmt, io, result};

use futures::stream::{SplitSink, SplitStream};
use futures::{
    future, FutureExt, SinkExt, Stream, StreamExt, TryFutureExt, TryStream, TryStreamExt,
};
use log::error;
use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::{tungstenite, WebSocketStream};
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::proto::{Input, InputMessage, Output, OutputMessage, SendInput};

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
        let client_id = self.id.clone();
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
        let client_id = self.id.clone();
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
