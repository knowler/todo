use crate::task::Task;
use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    name: String,
    opened: String,
    tasks: Vec<Task>,
}

impl Project {
    /// Create a project
    pub fn create(name: String) -> Self {
        let tasks = Vec::new();
        let opened = Utc::now().to_rfc3339();

        Self {
            name,
            opened,
            tasks,
        }
    }

    /// Get the project file
    pub fn get_file(&self) -> std::io::Result<File> {
        // Check if the file exists and open it
        if Path::new(&self.name).is_file() {
            return File::open(&self.name);
        }

        // If the file doesn't exist, create one.
        File::create(&self.name)
    }

    /// Get the project from file
    pub fn get_project(&self) -> Self {
        let mut file = &self.get_file().unwrap();

        let contents = std::fs::read_to_string(&self.name).unwrap();

        toml::from_str(contents.as_str()).unwrap()
    }

    /// Save the project file
    pub fn save(&self) -> std::io::Result<()> {
        // Retrieve the project file
        let mut file = &self.get_file().unwrap();

        // Check if the file has any contents
        // If no, create the TOML string
        // If yes, parse the TOML string and

        // Convert the struct to TOML
        let toml = toml::to_string(&self).unwrap();

        // Write TOML to file
        file.write_all(toml.as_bytes()).unwrap();

        Ok(())
    }

    /// Add a task to the project
    pub fn add_task(&mut self, task: Task) {
        &self.tasks.push(task);

        println!("{:?}", &self.tasks);
    }
}
