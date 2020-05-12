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

#[derive(Debug, Clone)]
pub struct OutputMessage {
    output: Output,
    target_client_id: Option<Uuid>,
    ignored_client_id: Option<Uuid>,
}

impl OutputMessage {
    pub fn new(output: Output) -> Self {
        OutputMessage {
            output,
            target_client_id: None,
            ignored_client_id: None,
        }
    }

    pub fn new_target(client_id: Uuid, output: Output) -> Self {
        OutputMessage {
            output,
            target_client_id: Some(client_id),
            ignored_client_id: None,
        }
    }

    pub fn new_ignored(client_id: Uuid, output: Output) -> Self {
        OutputMessage {
            output,
            target_client_id: None,
            ignored_client_id: Some(client_id),
        }
    }

    pub fn is_target(&self, client_id: Uuid) -> bool {
        Some(client_id) == self.target_client_id
            || self.target_client_id.is_none() && Some(client_id) != self.ignored_client_id
    }

    pub fn output(&self) -> &Output {
        &self.output
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
