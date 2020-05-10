use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    id: Uuid,
    name: String,
}

impl User {
    pub fn new(id: Uuid, name: String) -> Self {
        User { id, name }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
