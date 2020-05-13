use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum Input {
    #[serde(rename = "join")]
    Join(JoinInput),
    #[serde(rename = "post")]
    Post(PostInput),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Output {
    #[serde(rename = "error")]
    Error(OutputError),
    #[serde(rename = "alive")]
    Alive,
    #[serde(rename = "joined")]
    Joined(JoinedOutput),
    #[serde(rename = "posted")]
    Posted(PostedOutput),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "code", rename_all = "camelCase")]
pub enum OutputError {
    NameTaken,
    InvalidName,
    NotJoined,
    InvalidMessageBody,
}

#[derive(Debug, Clone)]
pub struct InputMessage {
    pub client_id: Uuid,
    pub input: Input,
}

impl InputMessage {
    pub fn new(client_id: Uuid, input: Input) -> Self {
        InputMessage { client_id, input }
    }
}

#[derive(Debug, Clone)]
pub struct OutputMessage {
    pub output: Output,
    pub target_client_id: Option<Uuid>,
    pub ignored_client_id: Option<Uuid>,
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
}

// {"type":"join","payload":{"name":"John"}}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinInput {
    pub name: String,
}

// {"type":"post","payload":{"body":"Hey!"}}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostInput {
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinedOutput {
    pub user_id: Uuid,
}

impl JoinedOutput {
    pub fn new(user_id: Uuid) -> Self {
        JoinedOutput { user_id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostedOutput {
    pub body: String,
}

impl PostedOutput {
    pub fn new(body: &str) -> Self {
        PostedOutput {
            body: String::from(body),
        }
    }
}
