//! Rusty-TODO-CLI
//! A command-line TODO application in Rust

use std::collections::HashMap;

struct ToDo {
    // Stores a key-value pair of a to-do item and its status
    // using the built-in `HashMap` of Rust.
    map: HashMap<String, bool>,
}

impl ToDo {
    fn create_new() -> Result<ToDo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        match serde_json::from_reader(f) {
            Ok(map) => Ok(ToDo { map }),
            Err(e) if e.is_eof() => Ok(ToDo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("There was a problem: {}.", e),
        }
    }

    fn insert(&mut self, key: String) {
        // Inserts a new to-do activity in the map while
        // setting the status/state of the activity to `true` by default
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json")?;
        serde_json::to_writer_pretty(f, &self.map)?;

        Ok(())
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(val) => Some(*val = false),
            None => None,
        }
    }
}

fn main() {
    let action = std::env::args().nth(1).expect(
        "Kindly provide an antion point:
                                                'add', or 'complete' ",
    );
    let item = std::env::args()
        .nth(2)
        .expect("Kindly provide a to-do activity");

    let mut todo = ToDo::create_new().expect("Failed to initialize the database");

    // Checks to see if an `action` point and a to-do `item` have been provided.
    // If true, the item is saved to the TODO database
    if action == "add" && item != "" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("To-do activity saved to the TODO database"),
            Err(e) => println!("There was a problem: {}. Please try again", e),
        }
    } else if action == "complete" && item != "" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the TODO database", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("TODO database modified"),
                Err(e) => println!("There was a problem: {}. Please try again", e),
            },
        }
    } else {
        panic!("Invalid command. Please restart the app and try again!");
    }
}
