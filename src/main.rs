use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, stdout, BufRead, BufReader, Error, ErrorKind, Write};

#[derive(Debug)]
struct Task {
    task_description: String,
    is_completed: bool
}

impl Task {
    fn new(task_description: String) -> Self {
        Task {
            task_description,
            is_completed: false
        }
    }
}


struct TodoList {
    tasks: HashMap<u32, Task>
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            tasks: HashMap::new()
        }
    }

    fn add_task(&mut self, task_description: String) {
        let task_id = self.tasks
            .keys().max().map(|max_id| { max_id + 1 })
            .unwrap_or(1);
        self.tasks.insert(task_id, Task::new(task_description));
    }

    fn delete_task(&mut self, task_id: u32) -> Result<(), ()> {
        match self.tasks.remove(&task_id) {
            Some(_) => Ok(()),
            None => Err(())
        }
    }

    fn edit_task(&mut self, task_id: u32, new_task_description: String) -> Result<(), ()> {
        match self.tasks.get_mut(&task_id) {
            Some(task) => {
                task.task_description = new_task_description;
                Ok(())
            }
            None => Err(())
        }
    }

    fn mark_as_completed(&mut self, task_id: u32) -> Result<(), ()> {
        match self.tasks.get_mut(&task_id) {
            Some(task) => {
                task.is_completed = true;
                Ok(())
            }
            None => Err(())
        }
    }

    fn save_to_file(&self, file_name: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_name)?;

        for (task_id, task) in &self.tasks {
            writeln!(
                file,
                "{}|||{}|||{}",
                task_id, task.task_description, task.is_completed
            )?;
        }

        Ok(())
    }

    fn load_from_file(&mut self, file_name: &str) -> io::Result<()> {
        self.tasks.clear();

        let file = File::open(file_name)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            match line {
                Ok(task_string) => {
                    let parts: Vec<&str> = task_string.split("|||").collect();
                    if parts.len() != 3 {
                        return Err(Error::from(ErrorKind::InvalidData));
                    }

                    let task_id = parts[0].parse::<u32>()
                        .map_err(|_| Error::from(ErrorKind::InvalidData))?;

                    if self.tasks.contains_key(&task_id) {
                        return Err(Error::from(ErrorKind::InvalidData));
                    }

                    let task_description = parts[1].to_string();
                    let is_completed = parts[2].parse::<bool>()
                        .map_err(|_| Error::from(ErrorKind::InvalidData))?;

                    self.tasks.insert(
                        task_id,
                        Task {
                            task_description,
                            is_completed
                        }
                    );
                },
                Err(_) => return Err(Error::from(ErrorKind::InvalidData))
            }
        }

        Ok(())
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    let file_name = "todo_list.txt";

    loop {
        println!("TODO LIST:");
        if todo_list.tasks.is_empty() {
            println!("  right now todo list is empty...")
        } else {
            for (task_id, task) in &todo_list.tasks {
                println!(
                    "  task id: {}; description: {}; is completed: {}",
                    task_id, task.task_description, task.is_completed
                )
            }
        }

        println!("\nACTIONS:");
        println!("  1. add new task");
        println!("  2. delete task");
        println!("  3. edit task");
        println!("  4. mark task as completed");
        println!("  5. export todo list to file");
        println!("  6. import todo list from file");
        println!("  7. exit");

        print!("\nselect action by entering the corresponding number: ");
        stdout().flush().unwrap();
        let mut action_input = String::new();
        match io::stdin().read_line(&mut action_input) {
            Ok(_) => { action_input = action_input.trim().to_string() },
            Err(_) => {
                println!("  error occurred while reading input. try again...");
                continue;
            }
        };
        let action: u32 = match action_input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("  invalid action choice. a number must be entered...\n\n");
                continue;
            }
        };

        match action {
            1 => {
                print!("  enter task description: ");
                stdout().flush().unwrap();
                let mut description = String::new();
                match io::stdin().read_line(&mut description) {
                    Ok(_) => { description = description.trim().to_string() },
                    Err(_) => {
                        println!("    error occurred while reading task description...\n\n");
                        continue;
                    }
                };
                todo_list.add_task(description.trim().to_string());
                println!("    task was added successfully\n\n");
            },
            2 => {
                if todo_list.tasks.is_empty() {
                    println!("  there's no tasks in todo list...\n\n")
                } else {
                    print!("  enter id of the task you want to delete: ");
                    stdout().flush().unwrap();
                    let mut task_id_input = String::new();
                    match io::stdin().read_line(&mut task_id_input) {
                        Ok(_) => { task_id_input = task_id_input.trim().to_string() },
                        Err(_) => {
                            println!("    error occurred while reading id of the task...\n\n");
                            continue;
                        }
                    };
                    let task_id: u32 = match task_id_input.trim().parse() {
                        Ok(number) => number,
                        Err(_) => {
                            println!("    invalid id of the task. a number must be entered...\n\n");
                            continue;
                        }
                    };
                    if todo_list.tasks.contains_key(&task_id) {
                        match todo_list.delete_task(task_id) {
                            Ok(_) => {
                                println!("    task #{task_id} was deleted successfully\n\n");
                                continue;
                            },
                            Err(_) => {
                                println!("    something went wrong. task #{task_id} wasn't deleted...\n\n");
                                continue;
                            }
                        };
                    } else {
                        println!("    no task found with id #{task_id}\n\n");
                        continue;
                    }
                }
            },
            3 => {
                if todo_list.tasks.is_empty() {
                    println!("  there's no tasks in todo list...\n\n")
                } else {
                    print!("  enter id of the task you want to edit: ");
                    stdout().flush().unwrap();
                    let mut task_id_input = String::new();
                    match io::stdin().read_line(&mut task_id_input) {
                        Ok(_) => { task_id_input = task_id_input.trim().to_string() },
                        Err(_) => {
                            println!("    error occurred while reading id of the task...\n\n");
                            continue;
                        }
                    };
                    let task_id: u32 = match task_id_input.trim().parse() {
                        Ok(number) => number,
                        Err(_) => {
                            println!("    invalid id of the task. a number must be entered...\n\n");
                            continue;
                        }
                    };
                    if todo_list.tasks.contains_key(&task_id) {
                        print!("    enter new task description: ");
                        stdout().flush().unwrap();
                        let mut new_description = String::new();
                        match io::stdin().read_line(&mut new_description) {
                            Ok(_) => { new_description = new_description.trim().to_string() },
                            Err(_) => {
                                println!("      error occurred while reading new task description...\n\n");
                                continue;
                            }
                        };
                        match todo_list.edit_task(task_id, new_description) {
                            Ok(_) => {
                                println!("      task #{task_id} was edited successfully\n\n")
                            },
                            Err(_) => {
                                println!("      something went wrong. task #{task_id} wasn't edited...\n\n")
                            }
                        };
                    } else {
                        println!("      no task found with id #{task_id}\n\n");
                    }
                }
            },
            4 => {
                if todo_list.tasks.is_empty() {
                    println!("  there's no tasks in todo list...\n\n")
                } else {
                    print!("  enter id of the task you want to mark as completed: ");
                    stdout().flush().unwrap();
                    let mut task_id_input = String::new();
                    match io::stdin().read_line(&mut task_id_input) {
                        Ok(_) => { task_id_input = task_id_input.trim().to_string() },
                        Err(_) => {
                            println!("    error occurred while reading id of the task...\n\n");
                            continue;
                        }
                    };
                    let task_id: u32 = match task_id_input.trim().parse() {
                        Ok(number) => number,
                        Err(_) => {
                            println!("    invalid id of the task. a number must be entered...\n\n");
                            continue;
                        }
                    };
                    if todo_list.tasks.contains_key(&task_id) {
                        match todo_list.mark_as_completed(task_id) {
                            Ok(_) => {
                                println!("    task #{task_id} was marked as completed successfully\n\n");
                                continue;
                            },
                            Err(_) => {
                                println!("    something went wrong. task #{task_id} wasn't marked as completed...\n\n");
                                continue;
                            }
                        };
                    } else {
                        println!("    no task found with id #{task_id}\n\n");
                        continue;
                    }
                }
            },
            5 => {
                if todo_list.tasks.is_empty() {
                    println!("  there's no tasks in todo list...\n\n")
                } else {
                    match todo_list.save_to_file(file_name) {
                        Ok(_) => {
                            println!("  todo list was successfully exported to {file_name}\n\n")
                        },
                        Err(_) => {
                            println!("  something went wrong. todo list wasn't exported to {file_name}\n\n")
                        }
                    }
                }
            },
            6 => {
                match todo_list.load_from_file(file_name) {
                    Ok(_) => println!("  todo list was successfully imported from {file_name}\n\n"),
                    Err(_) => println!("  something went wrong. todo list wasn't imported from {file_name}\n\n")
                }
            },
            7 => {
                println!("  exiting...");
                break;
            },
            _ => {
                println!("  invalid action choice. please select a valid option...\n\n");
            }
        }
    }
}
