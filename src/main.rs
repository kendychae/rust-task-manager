use std::io::{self, Write};

/// A simple task manager program demonstrating Rust fundamentals
/// including structs, enums, vectors, and pattern matching

#[derive(Debug, Clone)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

impl Task {
    fn new(id: usize, description: String) -> Self {
        Task {
            id,
            description,
            completed: false,
        }
    }

    fn display(&self) {
        let status = if self.completed { "✓" } else { " " };
        println!("[{}] {}. {}", status, self.id, self.description);
    }
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String) {
        let task = Task::new(self.next_id, description);
        self.tasks.push(task);
        println!("✅ Task {} added successfully!", self.next_id);
        self.next_id += 1;
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📋 No tasks yet. Add your first task!");
        } else {
            println!("\n📋 Your Tasks:");
            println!("─────────────────────────────────────");
            for task in &self.tasks {
                task.display();
            }
            println!("─────────────────────────────────────");
        }
    }

    fn complete_task(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            println!("🎉 Task {} marked as complete!", id);
        } else {
            println!("❌ Task {} not found.", id);
        }
    }

    fn delete_task(&mut self, id: usize) {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            println!("🗑️  Task {} deleted.", id);
        } else {
            println!("❌ Task {} not found.", id);
        }
    }

    fn get_stats(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;
        
        println!("\n📊 Statistics:");
        println!("─────────────────────────────────────");
        println!("Total Tasks: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);
        println!("─────────────────────────────────────");
    }

    /// View the last N tasks using slicing - demonstrates Rust's slice feature
    fn view_recent_tasks(&self, count: usize) {
        let total = self.tasks.len();
        if total == 0 {
            println!("📋 No tasks yet.");
            return;
        }
        
        let start = if total > count { total - count } else { 0 };
        let recent_tasks = &self.tasks[start..];  // Slicing: creates a view into the vector
        
        println!("\n📋 Last {} task(s):", recent_tasks.len());
        println!("─────────────────────────────────────");
        for task in recent_tasks {
            task.display();
        }
        println!("─────────────────────────────────────");
    }
}

fn print_header() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║   🦀 Rust Task Manager 🦀              ║");
    println!("║   Learning Rust Programming            ║");
    println!("╚════════════════════════════════════════╝");
}

fn print_menu() {
    println!("\n📝 Menu:");
    println!("1. Add a new task");
    println!("2. List all tasks");
    println!("3. Complete a task");
    println!("4. Delete a task");
    println!("5. View statistics");
    println!("6. View recent tasks");
    println!("7. Exit");
    print!("\nChoose an option (1-7): ");
    io::stdout().flush().unwrap();
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    print_header();
    println!("\n👋 Welcome! This is my Rust learning project.");
    println!("As a single mom of two learning to code, I built this");
    println!("task manager to demonstrate Rust fundamentals.\n");

    let mut manager = TaskManager::new();
    
    // Add some demo tasks
    println!("🎯 Adding demo tasks to get you started...");
    manager.add_task("Learn Rust ownership and borrowing".to_string());
    manager.add_task("Complete CSE310 Module 1 assignment".to_string());
    manager.add_task("Build a small Rust project".to_string());

    loop {
        print_menu();
        let choice = get_user_input();

        match choice.as_str() {
            "1" => {
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                let description = get_user_input();
                if !description.is_empty() {
                    manager.add_task(description);
                } else {
                    println!("❌ Task description cannot be empty!");
                }
            }
            "2" => {
                manager.list_tasks();
            }
            "3" => {
                print!("Enter task ID to complete: ");
                io::stdout().flush().unwrap();
                let id = get_user_input();
                if let Ok(task_id) = id.parse::<usize>() {
                    manager.complete_task(task_id);
                } else {
                    println!("❌ Please enter a valid number!");
                }
            }
            "4" => {
                print!("Enter task ID to delete: ");
                io::stdout().flush().unwrap();
                let id = get_user_input();
                if let Ok(task_id) = id.parse::<usize>() {
                    manager.delete_task(task_id);
                } else {
                    println!("❌ Please enter a valid number!");
                }
            }
            "5" => {
                manager.get_stats();
            }
            "6" => {
                print!("How many recent tasks to view? (default 3): ");
                io::stdout().flush().unwrap();
                let input = get_user_input();
                let count = if input.is_empty() {
                    3
                } else {
                    input.parse::<usize>().unwrap_or(3)
                };
                manager.view_recent_tasks(count);
            }
            "7" => {
                println!("\n👋 Thanks for using Rust Task Manager!");
                println!("Keep coding and building amazing things! 🚀");
                break;
            }
            _ => {
                println!("❌ Invalid option. Please choose 1-7.");
            }
        }
    }
}
