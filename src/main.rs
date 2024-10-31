mod types;
use crate::types::DoDate;
use crate::types::Status;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Task {
    status: Status,
    title: String,
    description: String,
    time: DoDate,
}

impl Task {
    fn new(status: Status, title: String, description: String, time: DoDate) -> Task {
        Task {
            status,
            title,
            description,
            time,
        }
    }
}
fn main() {
    let mut tasks: HashMap<Status, Task> = HashMap::new();

    println!("choose your option!");
    loop {
        let mut _option = String::new();
        println!("1- create new task");
        println!("2- list of task");
        println!("3- update a task");
        println!("4- exit");

        io::stdin()
            .read_line(&mut _option)
            .expect("somethingg went wrong");

        match _option.trim() {
            "1" => create_new_task(&mut tasks),
            "2" => println!("2"),
            "3" => println!("3"),
            _ => break,
        }
    }
}

fn create_new_task(tasks: &mut HashMap<Status, Task>) {
    let new_task = Task::new(
        Status::ToDo,
        "title.trim().to_string()".to_string(),
        "description.trim().to_string()".to_string(),
        DoDate::Today,
    );

    tasks.insert(Status::ToDo, new_task);

    println!("{:?}", tasks)
}
