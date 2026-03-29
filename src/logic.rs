use comfy_table::{Table, presets::UTF8_FULL};
use crate::task::Task;

pub fn print_tasks(tasks: &[Task]){
    let mut table = Table::new();

    table.load_preset(UTF8_FULL).set_header(vec!["Title", "Priority", "Status"]);

    for task in tasks{
        table.add_row(vec![
            &task.title,
            &task.priority.to_string(),
            &task.status.to_string()
        ]);
    }

    println!("{table}");
}