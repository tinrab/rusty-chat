use std::{error, result};

use futures::stream::SplitStream;
use futures::{future, Stream, StreamExt, TryStream, TryStreamExt};
use log::error;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite, WebSocketStream};
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::proto::{InputParcel, OutputParcel};

#[derive(Clone, Copy, Default)]
pub struct Client {
    pub id: Uuid,
}

impl Client {
    pub fn new() -> Self {
        Client { id: Uuid::new_v4() }
    }

    pub fn read_input(
        &self,
        stream: SplitStream<WebSocketStream<TcpStream>>,
    ) -> impl Stream<Item = Result<InputParcel>> {
        let client_id = self.id;
        stream
            // Take only text messages
            .take_while(|message| {
                future::ready(if let Ok(message) = message {
                    message.is_text()
                } else {
                    false
                })
            })
            // Deserialize JSON messages into proto::Input
            .map(move |message| match message {
                Err(err) => Err(Error::WebSocket(err)),
                Ok(message) => {
                    let data = message.to_text().unwrap().trim();
                    let input = serde_json::from_str(data)?;
                    Ok(InputParcel::new(client_id, input))
                }
            })
    }

    pub fn write_output<S, E>(
        &self,
        stream: S,
    ) -> impl Stream<Item = tungstenite::Result<tungstenite::Message>>
    where
        S: TryStream<Ok = OutputParcel, Error = E> + Stream<Item = result::Result<OutputParcel, E>>,
        E: error::Error,
    {
        let client_id = self.id;
        stream
            // Skip irrelevant parcels
            .try_filter(move |output_parcel| future::ready(output_parcel.client_id == client_id))
            // Serialize to JSON
            .map_ok(|output_parcel| {
                let data = serde_json::to_string(&output_parcel.output).unwrap();
                tungstenite::Message::text(data)
            })
            .map_err(|err| {
                error!("Client write error: {}", err);
                tungstenite::Error::Http(tungstenite::http::StatusCode::INTERNAL_SERVER_ERROR)
            })
    }
}
