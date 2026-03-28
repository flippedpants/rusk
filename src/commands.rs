use crate::task::{Task, Priority, Status};
use crate::storage::{save_to_json, load_json};
use uuid::Uuid;
use clap::{Arg,arg, ArgAction, ArgMatches, Command, command};
use chrono::prelude::*;

pub fn getting_started(){
    let cmds: ArgMatches = command!()
    .about("This tool lets you manage your tasks efficiently.")
        .subcommand(
            Command::new("add")
            .about("Adds a new task")
                .arg(
                    arg!(-t --title <"Title of the task">)
                    // .short('t')
                    // .long("title")
                    .help("Sets the title of the task")
                    .required(true)
                )
                .arg(
                    arg!(-p --priority <"Low,Medium,High">) 
                    // .short('p')
                    // .long("priority")
                    .help("Sets task priority")
                    .required(true)
                )
        )
        .subcommand(
            Command::new("ls")
            .about("Lists all the tasks")
                .arg(
                    Arg::new("pending")
                    // .short('p')
                    .long("pending")
                    .action(ArgAction::SetTrue)
                    .help("shows pending tasks")
                )
                .arg(
                    Arg::new("completed")
                    .short('c')
                    .long("completed")
                    .action(ArgAction::SetTrue)
                    .help("Shows completed tasks")
                )
        )
        .arg(
            Arg::new("done")
            .short('d')
            .long("done")
            .help("Update the task status to completed")
        ).get_matches();

    let mut tasks: Vec<Task> = load_json();

    match cmds.subcommand(){
        Some(("add", sub_arg)) =>{
            let title = sub_arg.get_one::<String>("title").unwrap();
            let priority: Priority = sub_arg.get_one::<String>("priority").unwrap().parse().expect("Enter a valid priority");
            let status: Status = String::from("Pending").parse().expect("Should a valid status");

            let local: DateTime<Local> = Local::now();
            let id = Uuid::new_v4();


            let new_task: Task = Task::new(id,String::from(title),priority,status,local);
            tasks.push(new_task);

            save_to_json(tasks);
        }
        Some(("ls", sub_arg)) => {
            let pending = sub_arg.get_flag("pending");              
            let completed = sub_arg.get_flag("completed");

            for task in tasks {
                if pending && completed {
                    println!("Cannot use both --pending and --completed");
                    return;
                }
                else if !pending && !completed {
                    println!("{:#?}", task)
                }

                if pending && task.status == Status::Pending {
                    println!("{:#?}", task);
                }
                else if completed && task.status == Status::Completed {
                    println!("{:#?}", task);
                }
            }
        }
        _ => {
            println!("No valid command was provided");
        }

    }
    {
        let mut tasks = load_json();
        let title = cmds.get_one::<String>("done").unwrap().to_string();
        let mut flag = false;

        for task in &mut tasks {
            if task.title == title && task.status == Status::Completed{
                println!("The task is already completed");
            }

            if task.title == title {
                task.status = Status::Completed;
                flag = true;
                println!("Status updated");
            }
        }

        if !flag{
            println!("Task does not exist!");
        }
    }
}