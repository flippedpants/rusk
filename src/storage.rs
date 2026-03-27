use crate::task::Task;
// use serde_json::Result;
use core::panic;
use std::fs::File;
use std::io::{Write, BufWriter};
use std::path::Path;

// static path = Path::new("list.json");

pub fn save_to_json(tasks: Vec<Task>){
    let path = Path::new("list.json");
    let display = path.display();

    let file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}, {}", display, why),
        Ok(file) => file,
    };

    // let j = serde_json::to_string(&tasks);

    let mut buf = BufWriter::new(file);
    serde_json::to_writer(&mut buf, &tasks);

    buf.flush();

    println!("All data saved to - {}", display);
}

pub fn load_json() -> Vec<Task>{
    let path = Path::new("list.json");
    
}