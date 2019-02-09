pub struct Project {
    name: String,
}

impl Project {
    pub fn create(name: String) -> Self {
        Self { name }
    }
}
