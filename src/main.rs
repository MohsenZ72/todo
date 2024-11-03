mod types;
use crate::types::{DoDate, Status};
use std::io;
use std::vec::Vec;

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
    let mut tasks: Vec<Task> = Vec::new();

    println!("Choose your option!");
    loop {
        let mut option = String::new();
        println!("1 - Create new task");
        println!("2 - List of tasks");
        println!("3 - Update a task");
        println!("4 - Exit");

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        match option.trim() {
            "1" => create_new_task(&mut tasks),
            "2" => task_list(&mut tasks),
            "3" => update_task(&mut tasks),
            "4" => break,
            _ => println!("Invalid option! Please try again."),
        }
    }
}

fn create_new_task(tasks: &mut Vec<Task>) {
    let mut title = String::new();
    println!("What is the task title:");
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");

    let mut description = String::new();
    println!("What is the task description:");
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");

    let mut status_input = String::new();
    println!("What is the task status (ToDo, Pending, InProgress, Done):");
    io::stdin()
        .read_line(&mut status_input)
        .expect("Failed to read line");

    let status = match status_input.trim() {
        "ToDo" => Status::ToDo,
        "Pending" => Status::Pending,
        "InProgress" => Status::InProgress,
        "Done" => Status::Done,
        _ => {
            println!("Invalid status, defaulting to ToDo");
            Status::ToDo
        }
    };

    let mut dodate_input = String::new();
    println!("What is the due date (Past, Today, Tomorrow, Later):");
    io::stdin()
        .read_line(&mut dodate_input)
        .expect("Failed to read line");

    let time = match dodate_input.trim() {
        "Past" => DoDate::Past,
        "Today" => DoDate::Today,
        "Tomorrow" => DoDate::Tomorrow,
        "Later" => DoDate::Later,
        _ => {
            println!("Invalid due date, defaulting to Today");
            DoDate::Today
        }
    };

    let new_task = Task::new(
        status,
        title.trim().to_string(),
        description.trim().to_string(),
        time,
    );
    tasks.push(new_task);

    task_list(tasks);
}

fn task_list(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks available.");
    } else {
        for (index, task) in tasks.iter().enumerate() {
            println!("{}: {:?}", index, task);
        }
    }
}

fn update_task(tasks: &mut Vec<Task>) {
    task_list(tasks);

    let mut update_option = String::new();
    println!("Choose your option:");
    println!("1 - Delete with index");
    println!("2 - Update status");
    println!("3 - Change due date");

    io::stdin()
        .read_line(&mut update_option)
        .expect("Failed to read line");

    match update_option.trim() {
        "1" => delete(tasks),
        "2" => update_status(tasks),
        "3" => update_time(tasks),
        _ => println!("Invalid option!"),
    }
}

fn delete(tasks: &mut Vec<Task>) {
    let mut index_input = String::new();
    println!("Enter the index of the task to delete:");
    io::stdin()
        .read_line(&mut index_input)
        .expect("Failed to read line");

    let index: usize = match index_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid index.");
            return;
        }
    };

    if index < tasks.len() {
        tasks.remove(index);
        println!("Task at index {} deleted.", index);
    } else {
        println!("Invalid index. No task deleted.");
    }
}

fn update_status(tasks: &mut Vec<Task>) {
    task_list(tasks);

    let mut index_input = String::new();
    println!("Enter the index of the task to update status:");
    io::stdin()
        .read_line(&mut index_input)
        .expect("Failed to read line");

    let index: usize = match index_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid index.");
            return;
        }
    };

    if index < tasks.len() {
        let mut status_input = String::new();
        println!("Enter new status (ToDo, Pending, InProgress, Done):");
        io::stdin()
            .read_line(&mut status_input)
            .expect("Failed to read line");

        let new_status = match status_input.trim() {
            "ToDo" => Status::ToDo,
            "Pending" => Status::Pending,
            "InProgress" => Status::InProgress,
            "Done" => Status::Done,
            _ => {
                println!("Invalid status. No update made.");
                return;
            }
        };

        tasks[index].status = new_status;
        println!("Updated task status to {:?}", tasks[index].status);
    } else {
        println!("Invalid index. No task updated.");
    }
}

fn update_time(tasks: &mut Vec<Task>) {
    task_list(tasks);

    let mut index_input = String::new();
    println!("Enter the index of the task to update due date:");
    io::stdin()
        .read_line(&mut index_input)
        .expect("Failed to read line");

    let index: usize = match index_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid index.");
            return;
        }
    };

    if index < tasks.len() {
        let mut dodate_input = String::new();
        println!("Enter new due date (Past, Today, Tomorrow, Later):");
        io::stdin()
            .read_line(&mut dodate_input)
            .expect("Failed to read line");

        let new_time = match dodate_input.trim() {
            "Past" => DoDate::Past,
            "Today" => DoDate::Today,
            "Tomorrow" => DoDate::Tomorrow,
            "Later" => DoDate::Later,
            _ => {
                println!("Invalid due date. No update made.");
                return;
            }
        };

        tasks[index].time = new_time;
        println!("Updated task due date to {:?}", tasks[index].time);
    } else {
        println!("Invalid index. No task updated.");
    }
}
