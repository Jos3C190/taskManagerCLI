![banner](https://i.ibb.co/c8CC1cP/A-adir-un-t-tulo.png)

This project provides a task management system developed in Rust, focused on offering an efficient and easy-to-use command line interface (CLI) for organizing and managing daily tasks.

## Features

- **Add Tasks**: Create new tasks with a name and description.
- **Mark Tasks as Completed**: Allows marking tasks as completed.
- **Unmark Completed Tasks**: Ability to revert the completion status of tasks.
- **Delete Tasks**: Deletes specific tasks or all tasks.
- **Edit Tasks**: Modify the details of existing tasks, including their name and description.
- **List Tasks**: Displays all tasks with details.


## CLI Usage
The command line interface (CLI) of taskmanager.exe offers a quick way to manage your tasks directly from the terminal. Below, you will find a list of available commands along with a brief description of what each one does. You can use these commands to add, edit, mark, and delete tasks, as well as to view all your current tasks.

To use a command, utilize the taskmanager.exe executable followed by the command and the required arguments. There is also the option to get additional help.
```bash
USAGE:
    taskmanager.exe [COMMAND]

COMMANDS:
    add       Add a new task
    mark      Mark one or all tasks as completed
    unmark    Unmark one or all completed tasks
    delete    Delete one task or all tasks
    edit      Edit an existing task
    list      List all tasks

OPTIONS:
  -h, --help     Print help
  -V, --version  Print version
```
### Usage Examples

#### Adding a Task

To add a new task, use the add command with the name and description of the task:

```bash
./taskmanager add --name "Task Name" --description "Task Description"
./taskmanager add -n "Task Name" -d "Task Description"
```

#### Deleting a Task

To delete a specific task, use the delete command with the task's ID. To delete all tasks, use `all`:

```bash
./taskmanager delete 123    # Delete the task with ID 123
./taskmanager delete all    # Delete all tasks
```

#### Marking a Task

To mark a task as completed, use the mark command with the task's ID. To mark all tasks, use `all`:

```bash
./taskmanager mark 123      # Mark the task with ID 123 as completed
./taskmanager mark all      # Mark all tasks as completed
```

#### Unmarking a Task

To revert the completion status of a task, use the unmark command with the task's ID. To unmark all tasks, use `all`:

```bash
./taskmanager unmark 123    # Unmark the task with ID 123
./taskmanager unmark all    # Unmark all tasks
```

#### Editing a Task

To edit an existing task, use the edit command with the task's ID and the new details:

```bash
./taskmanager edit 123 --name "New Name" --description "New Description"
./taskmanager edit 123 -n "New Name" -d "New Description"

./taskmanager edit 123 --name "New Name"
./taskmanager edit 123 --description "New Description"
```

#### Listing All Tasks

To list all tasks, use the list command:

```bash
./taskmanager list
```

## Libraries Used

- [Clap](https://crates.io/crates/clap): To create the CLI interface.
- [Chrono](https://crates.io/crates/chrono): Date and time management.
- [Serde](https://crates.io/crates/serde): Serialization and deserialization of data.
- [Dialoguer](https://crates.io/crates/dialoguer): Interactive user interface in the CLI.

