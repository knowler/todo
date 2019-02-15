use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
}

impl Task {
    pub fn create(name: String) -> Self {
        Self { name }
    }
}
