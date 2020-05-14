use std::collections::HashMap;
use std::time::Duration;

use chrono::Utc;
use futures::StreamExt;
use regex::Regex;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::{broadcast, RwLock};
use tokio::time;
use uuid::Uuid;

use crate::error::Result;
use crate::model::feed::Feed;
use crate::model::message::Message;
use crate::model::user::User;
use crate::proto::{
    Input, InputParcel, JoinInput, JoinedOutput, MessageOutput, Output, OutputError, OutputParcel,
    PostInput, UserJoinedOutput, UserLeftOutput, UserOutput, UserPostedOutput,
};

const OUTPUT_CHANNEL_SIZE: usize = 16;
const ALIVE_INTERVAL: Duration = Duration::from_secs(5);
lazy_static! {
    static ref USER_NAME_REGEX: Regex = Regex::new("[A-Za-z\\s]{4,24}").unwrap();
}

pub struct Hub {
    output_sender: broadcast::Sender<OutputParcel>,
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

    pub async fn run(&self, receiver: UnboundedReceiver<InputParcel>) -> Result<()> {
        let ticking_alive = self.tick_alive();
        let processing = receiver.for_each(|input_parcel| self.process(input_parcel));

        tokio::select! {
            result = ticking_alive => result,
            result = processing => result,
        }
        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<OutputParcel> {
        self.output_sender.subscribe()
    }

    pub async fn on_disconnect(&self, client_id: Uuid) {
        // Remove user on disconnect
        if self.users.write().await.remove(&client_id).is_some() {
            self.send_ignored(client_id, Output::UserLeft(UserLeftOutput::new(client_id)))
                .await;
        }
    }

    async fn process(&self, input_parcel: InputParcel) {
        match input_parcel.input {
            Input::Join(input) => self.process_join(input_parcel.client_id, input).await,
            Input::Post(input) => self.process_send(input_parcel.client_id, input).await,
        }
    }

    async fn process_join(&self, client_id: Uuid, input: JoinInput) {
        let user_name = input.name.trim();

        // Check if user's name is taken
        if self
            .users
            .read()
            .await
            .values()
            .any(|user| user.name == user_name)
        {
            self.send_error(client_id, OutputError::NameTaken);
            return;
        }

        // Validate user name
        if !USER_NAME_REGEX.is_match(user_name) {
            self.send_error(client_id, OutputError::InvalidName);
            return;
        }

        let user = User::new(client_id, user_name);
        self.users.write().await.insert(client_id, user.clone());

        // Report success to user
        self.send_targeted(
            client_id,
            Output::Joined(JoinedOutput::new(
                client_id,
                self.users
                    .read()
                    .await
                    .values()
                    .map(|user| UserOutput::new(user.id, &user.name))
                    .collect(),
                self.feed
                    .read()
                    .await
                    .messages_iter()
                    .map(|message| {
                        MessageOutput::new(
                            message.id,
                            UserOutput::new(message.user.id, &message.user.name),
                            &message.body,
                            message.created_at,
                        )
                    })
                    .collect(),
            )),
        );
        // Notify others that someone joined
        self.send_ignored(
            client_id,
            Output::UserJoined(UserJoinedOutput::new(UserOutput::new(client_id, user_name))),
        )
        .await;
    }

    async fn process_send(&self, client_id: Uuid, input: PostInput) {
        // Verify that user exists
        let user = if let Some(user) = self.users.read().await.get(&client_id) {
            user.clone()
        } else {
            self.send_error(client_id, OutputError::NotJoined);
            return;
        };

        // Validate message body
        if input.body.is_empty() || input.body.len() > 256 {
            self.send_error(client_id, OutputError::InvalidMessageBody);
            return;
        }

        let message = Message::new(Uuid::new_v4(), user.clone(), &input.body, Utc::now());
        self.feed.write().await.add_message(message.clone());

        // Notify everybody about new message
        self.send(Output::UserPosted(UserPostedOutput::new(
            MessageOutput::new(
                message.id,
                UserOutput::new(user.id, &user.name),
                &message.body,
                message.created_at,
            ),
        )))
        .await;
    }

    async fn tick_alive(&self) {
        let mut interval = time::interval(ALIVE_INTERVAL);
        loop {
            interval.tick().await;
            self.send(Output::Alive).await;
        }
    }

    async fn send(&self, output: Output) {
        if self.output_sender.receiver_count() == 0 {
            return;
        }
        self.users.read().await.keys().for_each(|user_id| {
            self.output_sender
                .send(OutputParcel::new(*user_id, output.clone()))
                .unwrap();
        });
    }

    fn send_targeted(&self, client_id: Uuid, output: Output) {
        if self.output_sender.receiver_count() > 0 {
            self.output_sender
                .send(OutputParcel::new(client_id, output))
                .unwrap();
        }
    }

    async fn send_ignored(&self, ignored_client_id: Uuid, output: Output) {
        if self.output_sender.receiver_count() == 0 {
            return;
        }
        self.users
            .read()
            .await
            .values()
            .filter(|user| user.id != ignored_client_id)
            .for_each(|user| {
                self.output_sender
                    .send(OutputParcel::new(user.id, output.clone()))
                    .unwrap();
            });
    }

    fn send_error(&self, client_id: Uuid, error: OutputError) {
        self.send_targeted(client_id, Output::Error(error));
    }
}

impl Default for Hub {
    fn default() -> Self {
        Self::new()
    }
}
