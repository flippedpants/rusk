use crate::task::{Task, Priority, Status};
use crate::storage::save_to_json;
use uuid::Uuid;
use clap::{Arg, ArgMatches, command, Command};
use chrono::prelude::*;

// pub enum Commands{
//     Add,
//     Update,
//     Delete,
// }

pub fn getting_started(){
    let cmds: ArgMatches = command!()
    .about("This tool lets you manage your tasks efficiently.")
        .subcommand(
            Command::new("add")
            .about("Adds a new task")
                .arg(
                    Arg::new("title")
                    .short('t')
                    .long("title")
                    .help("Describe your task")
                    .required(true)
                )
                .arg(
                    Arg::new("priority")
                    .short('p')
                    .long("priority")
                    .help("Sets task priority")
                    .required(true)
                )
        ).get_matches();

    let mut tasks: Vec<Task> = Vec::new();

    match cmds.subcommand(){
        Some(("add", sub_arg)) =>{
            let title = sub_arg.get_one::<String>("title").unwrap();
            let priority: Priority = sub_arg.get_one::<String>("priority").unwrap().parse().expect("Enter a valid priority");
            let status: Status = String::from("Pending").parse().expect("Should a valid status");

            let local: DateTime<Local> = Local::now();
            let id = Uuid::new_v4();


            let new_task: Task = Task::new(id,String::from(title),priority,status,local);
            tasks.push(new_task);


            println!("Task added:");
            println!("{:#?}", tasks);

            save_to_json(tasks);
        }
        _ => {
            println!("No valid command was provided");
        }
    }


}