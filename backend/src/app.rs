use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct App {
    books: Vec<String>,
}

impl App {
    fn new() -> App {
        App { books: Vec::new() }
    }

    pub fn init() -> App {
        App::from_file("app.json")
    }

    pub fn deinit(&self) {
        &self.to_file("app.json");
    }

    fn from_file(file_path: &str) -> App {
        if std::path::Path::new(file_path).exists() {
            let mut file = File::open(file_path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let app: App = serde_json::from_str(&contents).unwrap();
            app
        } else {
            let app = App::new();
            app.to_file(file_path);
            app
        }
    }

    fn to_file(&self, file_path: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)
            .unwrap();
        let json = serde_json::to_string(&self).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}
