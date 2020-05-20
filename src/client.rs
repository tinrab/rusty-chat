use std::{error, result};

use futures::stream::SplitStream;
use futures::{future, Stream, StreamExt, TryStream, TryStreamExt};
use uuid::Uuid;
use warp::filters::ws::WebSocket;

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
        stream: SplitStream<WebSocket>,
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
                Err(err) => Err(Error::System(err.to_string())),
                Ok(message) => {
                    let input = serde_json::from_str(message.to_str().unwrap())?;
                    Ok(InputParcel::new(client_id, input))
                }
            })
    }

    pub fn write_output<S, E>(&self, stream: S) -> impl Stream<Item = Result<warp::ws::Message>>
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
                warp::ws::Message::text(data)
            })
            .map_err(|err| Error::System(err.to_string()))
    }
}
