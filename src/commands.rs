use clap::{Arg, ArgMatches, command, Command};

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

    match cmds.subcommand(){
        Some(("add", sub_arg)) =>{
            let title = sub_arg.get_one::<String>("title").unwrap();
            let priority = sub_arg.get_one::<String>("priority").unwrap();
            println!("Task added:");
            println!("Task title- {}", title);
            println!("Task priority - {}", priority);
        }
        _ => {
            println!("No valid command was provided");
        }
    }
}