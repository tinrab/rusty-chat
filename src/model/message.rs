use uuid::Uuid;
use chrono::prelude::*;

#[derive(Debug, Clone)]
pub struct Message {
    id: Uuid,
    user_id: Uuid,
    body: String,
    created_at: DateTime<Utc>,
}

impl Message {
    pub fn new(id: Uuid, user_id: Uuid, body: String, created_at: DateTime<Utc>) -> Self {
        Message {
            id,
            user_id,
            body,
            created_at,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}
