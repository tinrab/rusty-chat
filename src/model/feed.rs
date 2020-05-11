use crate::model::message::Message;

pub struct Feed {
    messages: Vec<Message>,
}

impl Feed {
    pub fn new() -> Self {
        Feed {
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        self.messages
            .sort_by_key(|message| message.created_at().clone())
    }

    pub fn messages_iter(&self) -> impl Iterator<Item = &Message> {
        self.messages.iter()
    }
}
