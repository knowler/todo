extern crate dirs;

use std::fs::DirBuilder;
use std::path::Path;

pub struct App;

impl App {
    pub fn new() {
        match dirs::data_local_dir() {
            Some(path) => {
                let data = path.display().to_string() + "/knowler.todos";

                if Path::new(&data).exists() {
                    println!("Todos storage exists at {}", &data);
                } else {
                    DirBuilder::new().recursive(true).create(&data).unwrap();
                    println!("Created todos storage at {}", &data);
                }
            }
            None => println!("Can't find local data directory!"),
        }
    }
}
