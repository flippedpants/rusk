use crate::task::Task;
use core::panic;
use std::fs::{File, read_to_string};
use std::io::{Write, BufWriter};
use std::path::Path;

pub fn save_to_json(tasks: Vec<Task>){
    let path = Path::new("list.jsonl");
    let display = path.display();

    let file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}, {}", display, why),
        Ok(file) => file,
    };

    let mut buf = BufWriter::new(file);

    for task in tasks{
        let j = serde_json::to_string(&task).expect("failed to serialize");
        writeln!(buf, "{}", j).expect("failed to write");

        // println!("Task added:");
        // println!("{:#?}", task);
    }

    buf.flush();

    println!("All data saved to - {}", display);
}

pub fn load_json() -> Vec<Task>{
    let path = Path::new("list.jsonl");
    let mut tasks = Vec::new();

    if !path.exists(){
        return tasks;
    }

    for line in read_to_string(path).unwrap().lines() {
        let r: Task = serde_json::from_str(line).expect("Unable to load json");
        tasks.push(r);
    };

    return tasks;
}
