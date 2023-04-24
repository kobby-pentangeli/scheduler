use crate::{error::Error, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Read, Write},
};

/// Represents a container for managing scheduled tasks
#[derive(Debug, Deserialize, Serialize)]
pub struct TaskManager {
    // A map of scheduled tasks, where the keys represent the tasks,
    // and the values are booleans indicating whether the tasks are completed.
    // A value `true` means the task is "completed".
    tasks: HashMap<String, bool>,
}

impl TaskManager {
    /// Creates a new `TaskManager`.
    ///
    /// If a file named "db.json" exists, it attempts to read from it and deserialize
    /// the data into a `TaskManager`. Otherwise, an empty `TaskManager` is created.
    pub fn new() -> Result<Self> {
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json");

        match f {
            Ok(file) => match serde_json::from_reader::<std::fs::File, HashMap<String, bool>>(file)
            {
                Ok(tasks) => Ok(Self { tasks }),
                Err(e) if e.is_eof() => Ok(Self {
                    tasks: HashMap::new(),
                }),
                Err(e) => Err(Error::SerdeJsonSerializeError(e)),
            },
            Err(e) => Err(Error::FileCreateAndReadError(e)),
        }
    }

    /// Inserts a key-value pair into the map of tasks.
    ///
    /// If the map did not have this key present, `None` is returned.
    /// If the map did have this key present, the value is updated, and
    /// the old value is returned. The key is not updated, though.
    pub fn insert_task(&mut self, key: &str) -> Option<bool> {
        self.tasks.insert(key.to_string(), false)
    }

    /// Saves the current state of the `TaskManager` to a file named "db.json".
    pub fn save_task_to_db(&self) -> Result<()> {
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json");

        match f {
            Err(e) => Err(Error::FileCreateAndReadError(e)),
            Ok(file) => match serde_json::to_writer_pretty(file, &self.tasks) {
                Ok(_res) => Ok(()),
                Err(e) => Err(Error::SerdeJsonSerializeError(e)),
            },
        }
    }

    /// Removes a task from the `db.json` database
    pub fn delete_task_from_db(task: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        // Open the JSON file and parse it into a `serde_json::Value` object.
        let mut file = File::open("db.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let mut json: serde_json::Value = serde_json::from_str(&contents)?;

        // Delete the item from the `serde_json::Value` object.
        json.as_object_mut().unwrap().remove(task);

        // Write the modified `serde_json::Value` object back to the file.
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("db.json")?;
        file.write_all(serde_json::to_string_pretty(&json)?.as_bytes())?;

        Ok(())
    }

    /// Marks a task as completed.
    pub fn complete_task(&mut self, task: &str) -> Option<String> {
        match self.tasks.get_mut(task) {
            Some(val) => {
                *val = true;
                Some(format!("{} is now complete", task))
            }
            None => None,
        }
    }

    /// Iterates over the tasks, returning each key along with its status.
    /// If the value is true, it's considered incomplete, and if it's false, it's considered complete.
    pub fn display_all_tasks(&self) -> HashMap<&String, String> {
        let mut tasks = HashMap::new();
        for (key, value) in &self.tasks {
            let status = if *value {
                "Complete".to_string()
            } else {
                "Incomplete".to_string()
            };
            tasks.insert(key, status);
        }
        tasks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_manager_insert_task() {
        // Ensure that inserting a task into a `TaskManager` works as expected.
        let mut tm = TaskManager::new().unwrap();
        let task_name = "task1";
        tm.insert_task(task_name);
        assert_eq!(tm.tasks.get(task_name), Some(&false));
    }

    #[test]
    fn test_task_manager_save_task_to_db() {
        // Ensure that saving a `TaskManager` object to a file works as expected.
        let mut tm = TaskManager::new().unwrap();
        tm.insert_task("task1");
        assert!(tm.save_task_to_db().is_ok());
        let f = File::open("db.json").unwrap();
        let tasks: HashMap<String, bool> = serde_json::from_reader(f).unwrap();
        assert_eq!(tasks.get("task1"), Some(&false));
    }

    #[test]
    fn test_task_manager_delete_task_from_db() {
        // Ensure that deleting a task from the `db.json` file works as expected.
        let mut tm = TaskManager::new().unwrap();
        tm.insert_task("task1");
        assert!(tm.save_task_to_db().is_ok());
        TaskManager::delete_task_from_db("task1").unwrap();
        let f = File::open("db.json").unwrap();
        let tasks: HashMap<String, bool> = serde_json::from_reader(f).unwrap();
        assert_eq!(tasks.get("task1"), None);
    }

    #[test]
    fn test_task_manager_complete_task() {
        // Ensure that marking a task as `complete` works as expected.
        let mut tm = TaskManager::new().unwrap();
        tm.insert_task("task1");
        tm.complete_task("task1");
        assert_eq!(tm.tasks.get("task1"), Some(&true));
    }

    #[test]
    fn test_task_manager_display_all_tasks() {
        // Ensure that displaying all tasks works as expected.
        let mut tm = TaskManager::new().unwrap();
        tm.insert_task("task1");
        tm.insert_task("task2");
        let tasks = tm.display_all_tasks();
        assert_eq!(
            tasks.get(&"task1".to_string()),
            Some(&"Incomplete".to_string())
        );
        assert_eq!(
            tasks.get(&"task2".to_string()),
            Some(&"Incomplete".to_string())
        );
    }
}
