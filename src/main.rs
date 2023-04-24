use scheduler::TaskManager;

fn main() {
    let action = std::env::args()
        .nth(1)
        .expect("Kindly provide an action point: 'add', or 'complete' ");
    let task = std::env::args().nth(2).expect("Kindly provide a task");

    let mut task_manager = TaskManager::new().expect("Failed to initialize the database");

    // Check if an action point and a task have been provided.
    // If true, write the task to the database.
    if action == "add" && !task.is_empty() {
        task_manager.insert_task(&task);
        match task_manager.save_task_to_db() {
            Ok(_) => println!("Task saved to the database"),
            Err(e) => println!("There was a problem: {}. Please try again", e),
        }
    } else if action == "complete" && !task.is_empty() {
        match task_manager.complete_task(&task) {
            None => println!("'{:#?}' is not present in the database", &task),
            Some(_) => match task_manager.save_task_to_db() {
                Ok(_) => println!("Database modified"),
                Err(e) => println!("There was a problem: {}. Please try again", e),
            },
        }
    } else {
        panic!("Invalid command. Please restart the app and try again!");
    }

    // Show all tasks.
    println!("{:#?}", task_manager.display_all_tasks());
}
