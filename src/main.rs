mod task;
mod task_manager;

use clap::{Arg, Command};
use task_manager::TaskManager;

//-------------------------------------------
// CLI for the task manager
//-------------------------------------------

fn build_cli() -> Command {
    Command::new("Task Manager")
        .version("1.0")
        .author("José Carlos López Martínez")
        .about("This is a task manager where you can organize your tasks efficiently.")
        .subcommand(add_command())
        .subcommand(mark_command())
        .subcommand(unmark_command())
        .subcommand(delete_command())
        .subcommand(edit_command())
        .subcommand(list_command())
}

fn add_command() -> Command {
    Command::new("add")
        .about("Add a new task")
        .arg(Arg::new("name")
            .short('n')
            .long("name")
            .value_name("NAME")
            .help("Name of the task")
            .required(true))
        .arg(Arg::new("description")
            .short('d')
            .long("description")
            .value_name("DESCRIPTION")
            .help("Description of the task")
            .required(true))
}

fn mark_command() -> Command {
    Command::new("mark")
        .about("Mark one or all tasks as completed")
        .arg(Arg::new("target")
            .help("ID of the task to mark or 'all' to mark all tasks.")
            .required(true))
}

fn unmark_command() -> Command {
    Command::new("unmark")
        .about("Unmark one or all completed tasks")
        .arg(Arg::new("target")
            .help("ID of the task to unmark or 'all' to unmark all tasks.")
            .required(true))
}

fn delete_command() -> Command {
    Command::new("delete")
        .about("Delete a task or all tasks")
        .arg(Arg::new("target")
            .help("ID of the task to delete or 'all' to delete all tasks.")
            .required(true))
}

fn edit_command() -> Command {
    Command::new("edit")
        .about("Edit an existing task")
        .arg(Arg::new("id")
            .help("The ID of the task to edit")
            .required(true)
            .index(1)
            .value_name("ID"))
        .arg(Arg::new("name")
            .short('n')
            .long("name")
            .help("New name of the task")
            .value_name("NAME"))
        .arg(Arg::new("description")
            .short('d')
            .long("description")
            .help("New description of the task")
            .value_name("DESCRIPTION"))
}

fn list_command() -> Command {
    Command::new("list")
        .about("List all tasks")
}

//-------------------------------------------
// Command management
//-------------------------------------------

fn process_commands(manager: &mut TaskManager, matches: &clap::ArgMatches) {
    match matches.subcommand() {
        Some(("add", add_matches)) => process_add(manager, add_matches),
        Some(("mark", mark_matches)) => process_mark(manager, mark_matches),
        Some(("unmark", unmark_matches)) => process_unmark(manager, unmark_matches),
        Some(("delete", delete_matches)) => process_delete(manager, delete_matches),
        Some(("edit", edit_matches)) => process_edit(manager, edit_matches),
        Some(("list", _)) => manager.list_tasks(),
        _ => {}
    }
}

fn process_add(manager: &mut TaskManager, add_matches: &clap::ArgMatches) {
    let name = add_matches.get_one::<String>("name")
        .expect("No name was provided for the task")
        .to_string();

    let description = add_matches.get_one::<String>("description")
        .expect("No task description was provided.")
        .to_string();

    manager.add_task(name, description);
}

fn process_mark(manager: &mut TaskManager, mark_matches: &clap::ArgMatches) {
    let target = mark_matches.get_one::<String>("target")
        .expect("No target was provided");
    
        if target == "all" { manager.mark_all_tasks(); } 
        else if let Ok(id) = target.parse::<u32>() {
            manager.mark_task_completed(id);
        } else {
            println!("Invalid argument. Please provide a valid ID or 'all'.");
        }
}

fn process_unmark(manager: &mut TaskManager, unmark_matches: &clap::ArgMatches) {
    let target = unmark_matches.get_one::<String>("target")
        .expect("No target was provided");
    
        if target == "all" { manager.unmark_all_tasks(); } 
        else if let Ok(id) = target.parse::<u32>() {
            manager.unmark_task_completed(id);
        } else {
            println!("Invalid argument. Please provide a valid ID or 'all'.");
        }
}

fn process_delete(manager: &mut TaskManager, delete_matches: &clap::ArgMatches) {
    let target = delete_matches.get_one::<String>("target")
        .expect("No target was provided");
    
        if target == "all" { manager.delete_all_tasks(); } 
        else if let Ok(id) = target.parse::<u32>() {
            manager.delete_task(id);
        } else {
            println!("Invalid argument. Please provide a valid ID or 'all'.");
        }
}

fn process_edit(manager: &mut TaskManager, edit_matches: &clap::ArgMatches) {
    let id = edit_matches.get_one::<String>("id")
        .expect("Task ID is required")
        .parse::<u32>()
        .expect("ID must be a number");

    let new_name = edit_matches.get_one::<String>("name").cloned();
    let new_description = edit_matches.get_one::<String>("description").cloned();

    manager.edit_task(id, new_name, new_description);
}


fn main() {
    let mut manager = TaskManager::new();
    let app = build_cli();
    let matches = app.get_matches();
    process_commands(&mut manager, &matches);
}