use std::fs::DirBuilder;
use std::path::Path;

pub struct App;

impl App {
    pub fn new() {
        let path = "todos";

        if !Path::new(path).exists() {
            DirBuilder::new()
                .recursive(true)
                .create(path)
                .unwrap();
        }
    }
}
