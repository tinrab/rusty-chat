use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Input {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "send")]
    Send(SendInput),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Output {
    #[serde(rename = "alive")]
    Alive,
}

#[derive(Debug, Clone)]
pub struct InputMessage {
    client_id: Uuid,
    input: Input,
}

impl InputMessage {
    pub fn new(client_id: Uuid, input: Input) -> Self {
        InputMessage { client_id, input }
    }

    pub fn client_id(&self) -> &Uuid {
        &self.client_id
    }

    pub fn input(&self) -> &Input {
        &self.input
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendInput {
    body: String,
}

impl SendInput {
    pub fn new(body: &str) -> Self {
        SendInput {
            body: String::from(body),
        }
    }
}
