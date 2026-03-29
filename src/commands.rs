use crate::task::{Task, Priority, Status};
use crate::storage::{save_to_json, load_json};
use uuid::Uuid;
use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command, arg, command};
use chrono::prelude::*;

pub fn getting_started(){
    let cmds: ArgMatches = command!()
    .about("This tool lets you manage your tasks efficiently.")
        .subcommand(
            Command::new("add")
            .about("Adds a new task")
                .arg(
                    arg!(-t --title <"Title of the task">)
                    .help("Sets the title of the task")
                    .required(true)
                )
                .arg(
                    arg!(-p --priority <"Low,Medium,High">) 
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
        .subcommand(
            Command::new("done")
            .about("Marks the task completed")
                .arg(
                    arg!([TITLE] "Title of the task to update")
                    .help("Title of the task")
                    .required(true)
                )
        )
        .subcommand(
            Command::new("delete")
            .about("Deletes the task")
                .group(ArgGroup::new("selection").required(true))
                .arg(
                    arg!([TITLE] "Title of the task to delete")
                    .group("selection")
                )
                .arg(
                    arg!(-a --all "Deletes all the task")
                    .group("selection")
                    .action(ArgAction::SetTrue)
                )
                .arg(
                    arg!(-p --pending "Deletes all the pending task")
                    .group("selection")
                    .action(ArgAction::SetTrue)
                )
                .arg(
                    arg!(-c --completed "Deletes all the completed task")
                    .group("selection")
                    .action(ArgAction::SetTrue)
                )
            )
        .get_matches();

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

            if pending && completed {
                println!("Cannot use both --pending and --completed");
                return;
            }

            for task in tasks {
                if !pending && !completed {
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
        Some(("done", sub_arg)) => {
            let title = sub_arg.get_one::<String>("TITLE").unwrap().to_string();
            let mut flag = false;

            for task in &mut tasks {
                if task.title == title && task.status == Status::Completed{
                    println!("The task is already completed");
                    return;
                }
                else if task.title == title {
                    task.status = Status::Completed;
                    flag = true;
                    println!("Status updated");
                }
            }

            if !flag{
                println!("Task does not exist!");
            }
            
            save_to_json(tasks);
        }
        Some(("delete", sub_arg)) => {
            if let Some(title) = sub_arg.get_one::<String>("TITLE"){
                tasks.retain(|t| t.title != *title);
                println!("Task deleted.");
            }
            else if sub_arg.get_flag("completed"){
                tasks.retain(|t| t.status != Status::Completed);
                println!("Deleted the completed tasks!");
            }
            else if sub_arg.get_flag("pending"){
                tasks.retain(|t| t.status != Status::Pending);
                println!("Deleted all the pending tasks");
            }
            else if sub_arg.get_flag("all"){
                tasks.clear();
                println!("Deleted all the tasks.");
            }
            save_to_json(tasks);
        }   
        _ => {
            println!("No valid command was provided");
        }

    }
}