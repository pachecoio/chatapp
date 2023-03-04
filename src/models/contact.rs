use crate::adapters::Entity;

#[derive(Clone, Debug)]
pub struct Contact {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl Entity for Contact {
    fn id(&self) -> &str {
        &self.id
    }
}
