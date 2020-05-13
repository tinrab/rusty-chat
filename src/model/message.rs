use chrono::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Message {
    pub id: Uuid,
    pub user_id: Uuid,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

impl Message {
    pub fn new(id: Uuid, user_id: Uuid, body: &str, created_at: DateTime<Utc>) -> Self {
        Message {
            id,
            user_id,
            body: String::from(body),
            created_at,
        }
    }
}
