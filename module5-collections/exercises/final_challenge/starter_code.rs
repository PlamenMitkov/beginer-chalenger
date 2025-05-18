use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize, Clone)]
enum TaskStatus {
    Pending,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: u32,
    title: String,
    description: String,
    due_date: Option<String>,
    status: TaskStatus,
}

impl Task {
    fn display(&self) {
        let status = match self.status {
            TaskStatus::Pending => "🕒 Pending",
            TaskStatus::Completed => "✅ Completed",
        };
        let due_date = self.due_date.as_deref().unwrap_or("No due date");
        println!(
            "ID: {}\nTitle: {}\nDescription: {}\nDue: {}\nStatus: {}\n",
            self.id, self.title, self.description, due_date, status
        );
    }
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }
    
    fn add_task(&mut self, title: String, description: String, due_date: Option<String>) -> Task {
        let task = Task {
            id: self.next_id,
            title,
            description,
            due_date,
            status: TaskStatus::Pending,
        };
        
        self.next_id += 1;
        self.tasks.push(task.clone());
        task
    }
    
    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("\nNo tasks found.");
            return;
        }
        
        println!("\nTask List:");
        println!("----------");
        for task in &self.tasks {
            task.display();
        }
    }
    
    fn complete_task(&mut self, id: u32) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.status = TaskStatus::Completed;
            Ok(())
        } else {
            Err(format!("Task with ID {} not found", id))
        }
    }
    
    fn delete_task(&mut self, id: u32) -> Result<(), String> {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            Ok(())
        } else {
            Err(format!("Task with ID {} not found", id))
        }
    }
    
    fn filter_by_status(&self, completed: bool) -> Vec<&Task> {
        self.tasks.iter().filter(|task| {
            matches!(
                (completed, &task.status),
                (true, TaskStatus::Completed) | (false, TaskStatus::Pending)
            )
        }).collect()
    }
    
    fn filter_by_due_date(&self, due_date: &str) -> Vec<&Task> {
        self.tasks.iter()
            .filter(|task| task.due_date.as_ref().map_or(false, |d| d == due_date))
            .collect()
    }
    
    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let serialized = serde_json::to_string_pretty(&self.tasks)?;
        std::fs::write(filename, serialized)
    }
    
    fn load_from_file(&mut self, filename: &str) -> std::io::Result<()> {
        let data = std::fs::read_to_string(filename)?;
        let tasks: Vec<Task> = serde_json::from_str(&data)?;
        
        self.next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        self.tasks = tasks;
        Ok(())
    }
}

fn display_filtered_tasks(tasks: Vec<&Task>) {
    if tasks.is_empty() {
        println!("No tasks match the filter criteria.");
        return;
    }
    
    for task in tasks {
        task.display();
    }
}

enum Command {
    Add { title: String, description: String, due_date: Option<String> },
    List,
    Complete { id: u32 },
    Delete { id: u32 },
    Save { filename: String },
    Load { filename: String },
    FilterStatus { completed: bool },
    FilterDate { date: String },
    Quit,
    Unknown,
}

fn parse_command(input: &str) -> Command {
    let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();
    match parts[0].to_lowercase().as_str() {
        "add" => {
            if parts.len() < 2 {
                return Command::Unknown;
            }
            let details: Vec<&str> = parts[1].splitn(3, ',').collect();
            if details.len() < 2 {
                return Command::Unknown;
            }
            let title = details[0].trim().to_string();
            let description = details[1].trim().to_string();
            let due_date = if details.len() > 2 {
                Some(details[2].trim().to_string())
            } else {
                None
            };
            Command::Add { title, description, due_date }
        }
        "list" => Command::List,
        "complete" => {
            if parts.len() < 2 {
                return Command::Unknown;
            }
            match parts[1].trim().parse() {
                Ok(id) => Command::Complete { id },
                Err(_) => Command::Unknown,
            }
        }
        "delete" => {
            if parts.len() < 2 {
                return Command::Unknown;
            }
            match parts[1].trim().parse() {
                Ok(id) => Command::Delete { id },
                Err(_) => Command::Unknown,
            }
        }
        "save" => {
            if parts.len() < 2 {
                return Command::Unknown;
            }
            Command::Save { filename: parts[1].trim().to_string() }
        }
        "load" => {
            if parts.len() < 2 {
                return Command::Unknown;
            }
            Command::Load { filename: parts[1].trim().to_string() }
        }
        "filter" => {
            if parts.len() < 2 {
                return Command::Unknown;
            }
            let filter_parts: Vec<&str> = parts[1].splitn(2, ' ').collect();
            match filter_parts[0].to_lowercase().as_str() {
                "status" => {
                    if filter_parts.len() < 2 {
                        return Command::Unknown;
                    }
                    match filter_parts[1].to_lowercase().as_str() {
                        "completed" => Command::FilterStatus { completed: true },
                        "pending" => Command::FilterStatus { completed: false },
                        _ => Command::Unknown,
                    }
                }
                "date" => {
                    if filter_parts.len() < 2 {
                        return Command::Unknown;
                    }
                    Command::FilterDate { date: filter_parts[1].trim().to_string() }
                }
                _ => Command::Unknown,
            }
        }
        "quit" => Command::Quit,
        _ => Command::Unknown,
    }
}

fn print_help() {
    println!("\nAvailable commands:");
    println!("------------------");
    println!("  add <title>, <description>, [due date] - Add a new task");
    println!("  list                                    - List all tasks");
    println!("  complete <id>                          - Mark a task as completed");
    println!("  delete <id>                           - Delete a task");
    println!("  save <filename>                       - Save tasks to file");
    println!("  load <filename>                       - Load tasks from file");
    println!("  filter status <completed|pending>     - Filter tasks by status");
    println!("  filter date <date>                    - Filter tasks by due date");
    println!("  quit                                  - Exit the program");
    println!("  help                                  - Show this help message\n");
}

fn main() {
    let mut task_manager = TaskManager::new();
    
    println!("🔧 Rust Task Manager");
    println!("-------------------");
    println!("Type 'help' for a list of commands\n");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        let command = parse_command(&input);
        
        match command {
            Command::Add { title, description, due_date } => {
                let task = task_manager.add_task(title, description, due_date);
                println!("\n✨ Added new task:");
                task.display();
            }
            Command::List => {
                task_manager.list_tasks();
            }
            Command::Complete { id } => {
                match task_manager.complete_task(id) {
                    Ok(_) => println!("\n✅ Task {} marked as completed", id),
                    Err(e) => println!("\n❌ Error: {}", e),
                }
            }
            Command::Delete { id } => {
                match task_manager.delete_task(id) {
                    Ok(_) => println!("\n🗑️ Task {} deleted", id),
                    Err(e) => println!("\n❌ Error: {}", e),
                }
            }
            Command::Save { filename } => {
                match task_manager.save_to_file(&filename) {
                    Ok(_) => println!("\n💾 Tasks saved to {}", filename),
                    Err(e) => println!("\n❌ Error saving file: {}", e),
                }
            }
            Command::Load { filename } => {
                match task_manager.load_from_file(&filename) {
                    Ok(_) => {
                        println!("\n📂 Tasks loaded from {}", filename);
                        task_manager.list_tasks();
                    }
                    Err(e) => println!("\n❌ Error loading file: {}", e),
                }
            }
            Command::FilterStatus { completed } => {
                println!("\n🔍 Filtered tasks ({}):", 
                    if completed { "completed" } else { "pending" });
                let tasks = task_manager.filter_by_status(completed);
                display_filtered_tasks(tasks);
            }
            Command::FilterDate { date } => {
                println!("\n🔍 Tasks due on {}:", date);
                let tasks = task_manager.filter_by_due_date(&date);
                display_filtered_tasks(tasks);
            }
            Command::Quit => {
                println!("\n👋 Goodbye!");
                break;
            }
            Command::Unknown => {
                if input.trim().to_lowercase() == "help" {
                    print_help();
                } else {
                    println!("\n❓ Unknown command. Type 'help' for available commands.");
                }
            }
        }
    }
}