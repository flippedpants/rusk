use uuid::Uuid;

mod task;
use task::{Task,Priority,Status};

fn main() {
    let id = Uuid::new_v4();
    let priority: Priority = String::from("High").parse().expect("Enter a valid priority");
    let status: Status = String::from("Pending").parse().expect("Enter a valid status");

    let new_task = Task::new(id, String::from("Submit DA"), priority, status);

    println!("{:?}", new_task);
}
