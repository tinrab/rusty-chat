use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use chrono::Utc;
use futures::{Stream, StreamExt, TryStreamExt};
use regex::Regex;
use tokio::sync::{broadcast, RwLock};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::time;
use uuid::Uuid;

use crate::error::Result;
use crate::model::feed::Feed;
use crate::model::message::Message;
use crate::model::user::User;
use crate::proto::{Input, InputMessage, JoinedOutput, JoinInput, Output, OutputError, OutputMessage, PostedOutput, PostInput};

const OUTPUT_CHANNEL_SIZE: usize = 16;
const ALIVE_INTERVAL: time::Duration = time::Duration::from_secs(5);
lazy_static! {
    static ref USER_NAME_REGEX: Regex = Regex::new("[A-Za-z\\s]{4,24}").unwrap();
}

pub struct Hub {
    output_sender: broadcast::Sender<OutputMessage>,
    users: RwLock<HashMap<Uuid, User>>,
    feed: RwLock<Feed>,
}

impl Hub {
    pub fn new() -> Self {
        let (output_sender, _) = broadcast::channel(OUTPUT_CHANNEL_SIZE);
        Hub {
            output_sender,
            users: Default::default(),
            feed: Default::default(),
        }
    }

    pub async fn run(&self, receiver: UnboundedReceiver<InputMessage>) -> Result<()> {
        let ticking_alive = self.tick_alive();
        let processing = receiver.for_each(|input_message| self.process(input_message));

        tokio::select! {
            result = ticking_alive => result,
            result = processing => result,
        }
        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<OutputMessage> {
        self.output_sender.subscribe()
    }

    async fn process(&self, input_message: InputMessage) {
        match input_message.input {
            Input::Join(input) => self.process_join(input_message.client_id, input).await,
            Input::Post(input) => self.process_send(input_message.client_id, input).await,
        }
    }

    async fn process_join(&self, client_id: Uuid, input: JoinInput) {
        let mut users = self.users.write().await;
        if users.values().any(|user| user.name == input.name) {
            self.send_error(
                client_id,
                OutputError::NameTaken,
            );
            return;
        }

        if !USER_NAME_REGEX.is_match(&input.name) {
            self.send_error(client_id, OutputError::InvalidName);
            return;
        }

        let user = User::new(client_id, input.name);
        users.insert(client_id, user);

        self.send(OutputMessage::new_target(client_id, Output::Joined(JoinedOutput::new(
            client_id,
        ))));
    }

    async fn process_send(&self, client_id: Uuid, input: PostInput) {
        if !self.users.read().await.contains_key(&client_id) {
            self.send_error(client_id, OutputError::NotJoined);
            return;
        }

        if input.body.len() == 0 || input.body.len() > 256 {
            self.send_error(client_id, OutputError::InvalidMessageBody);
            return;
        }

        let message = Message::new(Uuid::new_v4(), client_id, &input.body, Utc::now());
        self.feed.write().await.add_message(message);

        self.send(OutputMessage::new(Output::Posted(PostedOutput::new(&input.body))));
    }

    async fn tick_alive(&self) {
        let mut interval = time::interval(ALIVE_INTERVAL);
        loop {
            interval.tick().await;
            self.send(OutputMessage::new(Output::Alive))
        }
    }

    fn send(&self, message: OutputMessage) {
        if self.output_sender.receiver_count() > 0 {
            self.output_sender.send(message).unwrap();
        }
    }

    fn send_error(&self, client_id: Uuid, error: OutputError) {
        self.send(OutputMessage::new_target(client_id, Output::Error(error)));
    }
}
