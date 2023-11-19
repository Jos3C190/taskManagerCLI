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
        .subcommand(delete_command())
        .subcommand(list_command())
}

fn add_command() -> Command {
    Command::new("add")
        .about("Add a new task")
        .arg(Arg::new("name")
            .help("Name of the task")
            .required(true))
        .arg(Arg::new("description")
            .help("Description of the task")
            .required(true))
}

fn mark_command() -> Command {
    Command::new("mark")
        .about("Mark a task as completed")
        .arg(Arg::new("index")
            .help("Index of the task")
            .required(true))
}

fn delete_command() -> Command {
    Command::new("delete")
        .about("Delete a task")
        .arg(Arg::new("index")
            .help("Index of the task")
            .required(true))
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
        Some(("delete", delete_matches)) => process_delete(manager, delete_matches),
        Some(("list", _)) => manager.list_tasks(),
        _ => {}
    }
}

fn process_add(manager: &mut TaskManager, add_matches: &clap::ArgMatches) {
    let name = add_matches.get_one::<String>("name").unwrap().to_string();
    let description = add_matches.get_one::<String>("description").unwrap().to_string();
    manager.add_task(name, description);
}

fn process_mark(manager: &mut TaskManager, mark_matches: &clap::ArgMatches) {
    let index = mark_matches.get_one::<String>("index").unwrap().parse::<usize>().expect("Índice inválido");
    manager.mark_task_completed(index);
}

fn process_delete(manager: &mut TaskManager, delete_matches: &clap::ArgMatches) {
    let index = delete_matches.get_one::<String>("index").unwrap().parse::<usize>().expect("Índice inválido");
    manager.delete_task(index);
}


fn main() {
    let mut manager = TaskManager::new();
    let app = build_cli();
    let matches = app.get_matches();
    process_commands(&mut manager, &matches);
}