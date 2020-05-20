use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum Input {
    #[serde(rename = "join")]
    Join(JoinInput),
    #[serde(rename = "post")]
    Post(PostInput),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Output {
    #[serde(rename = "error")]
    Error(OutputError),
    #[serde(rename = "alive")]
    Alive,
    #[serde(rename = "joined")]
    Joined(JoinedOutput),
    #[serde(rename = "user-joined")]
    UserJoined(UserJoinedOutput),
    #[serde(rename = "user-left")]
    UserLeft(UserLeftOutput),
    #[serde(rename = "posted")]
    Posted(PostedOutput),
    #[serde(rename = "user-posted")]
    UserPosted(UserPostedOutput),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "code")]
pub enum OutputError {
    #[serde(rename = "name-taken")]
    NameTaken,
    #[serde(rename = "invalid-name")]
    InvalidName,
    #[serde(rename = "not-joined")]
    NotJoined,
    #[serde(rename = "invalid-message-body")]
    InvalidMessageBody,
}

#[derive(Debug, Clone)]
pub struct InputParcel {
    pub client_id: Uuid,
    pub input: Input,
}

impl InputParcel {
    pub fn new(client_id: Uuid, input: Input) -> Self {
        InputParcel { client_id, input }
    }
}

#[derive(Debug, Clone)]
pub struct OutputParcel {
    pub client_id: Uuid,
    pub output: Output,
}

impl OutputParcel {
    pub fn new(client_id: Uuid, output: Output) -> Self {
        OutputParcel { client_id, output }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinInput {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostInput {
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserOutput {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageOutput {
    pub id: Uuid,
    pub user: UserOutput,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinedOutput {
    pub user: UserOutput,
    pub others: Vec<UserOutput>,
    pub messages: Vec<MessageOutput>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserJoinedOutput {
    pub user: UserOutput,
}

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLeftOutput {
    pub user_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostedOutput {
    pub message: MessageOutput,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPostedOutput {
    pub message: MessageOutput,
}

impl UserOutput {
    pub fn new(id: Uuid, name: &str) -> Self {
        UserOutput {
            id,
            name: String::from(name),
        }
    }
}

impl MessageOutput {
    pub fn new(id: Uuid, user: UserOutput, body: &str, created_at: DateTime<Utc>) -> Self {
        MessageOutput {
            id,
            user,
            body: String::from(body),
            created_at,
        }
    }
}

impl JoinedOutput {
    pub fn new(user: UserOutput, others: Vec<UserOutput>, messages: Vec<MessageOutput>) -> Self {
        JoinedOutput {
            user,
            others,
            messages,
        }
    }
}

impl UserJoinedOutput {
    pub fn new(user: UserOutput) -> Self {
        UserJoinedOutput { user }
    }
}

impl UserLeftOutput {
    pub fn new(user_id: Uuid) -> Self {
        UserLeftOutput { user_id }
    }
}

impl PostedOutput {
    pub fn new(message: MessageOutput) -> Self {
        PostedOutput { message }
    }
}

impl UserPostedOutput {
    pub fn new(message: MessageOutput) -> Self {
        UserPostedOutput { message }
    }
}
