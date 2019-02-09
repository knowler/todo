pub struct Task {
    name: String,
}

impl Task {
    pub fn create(name: String) -> Self {
        Self { name }
    }
}
