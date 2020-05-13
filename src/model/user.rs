use std::hash::{Hash, Hasher};

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

impl User {
    pub fn new(id: Uuid, name: &str) -> Self {
        User {
            id,
            name: String::from(name),
        }
    }
}
