use todo::Journal;

fn main() {
    let action = std::env::args()
        .nth(1)
        .expect("Kindly provide an antion point: 'add', or 'complete' ");
    let item = std::env::args()
        .nth(2)
        .expect("Kindly provide a to-do activity");

    let mut todo = Journal::new().expect("Failed to initialize the database");

    // Checks to see if an `action` point and a to-do `item` have been provided.
    // If true, the item is saved to the TODO database
    if action == "add" && !item.is_empty() {
        todo.insert(&item);
        match todo.save() {
            Ok(_) => println!("To-do activity saved to the TODO database"),
            Err(e) => println!("There was a problem: {}. Please try again", e),
        }
    } else if action == "complete" && !item.is_empty() {
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
