use std::io;
use std::collections::HashMap;

#[derive(Debug)]
struct Task {
    id: usize,
    description: String,
    done: bool,
}

struct TodoList {
    tasks: HashMap<usize, Task>,
    next_id: usize,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String) {
        let task = Task {
            id: self.next_id,
            description,
            done: false,
        };
        self.tasks.insert(self.next_id, task);
        println!("Task added with ID: {}", self.next_id);
        self.next_id += 1;
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks available.");
        } else {
            for task in self.tasks.values() {
                let status = if task.done { "✅ Done" } else { "❌ Pending" };
                println!("[{}] {} - {}", task.id, task.description, status);
            }
        }
    }

    fn mark_done(&mut self, id: usize) {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.done = true;
            println!("Task {} marked as done!", id);
        } else {
            println!("Task ID {} not found.", id);
        }
    }

    fn delete_task(&mut self, id: usize) {
        if self.tasks.remove(&id).is_some() {
            println!("Task {} deleted.", id);
        } else {
            println!("Task ID {} not found.", id);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    
    loop {
        println!("\n📌 To-Do List Menu:");
        println!("1️⃣ Add Task");
        println!("2️⃣ View Tasks");
        println!("3️⃣ Mark Task as Done");
        println!("4️⃣ Delete Task");
        println!("5️⃣ Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choice = input.trim();

        match choice {
            "1" => {
                println!("Enter task description:");
                let mut desc = String::new();
                io::stdin().read_line(&mut desc).expect("Failed to read input");
                let desc = desc.trim().to_string();  // ✅ FIXED LIFETIME ISSUE

                todo_list.add_task(desc);
            }
            "2" => todo_list.list_tasks(),
            "3" => {
                println!("Enter Task ID to mark as done:");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Failed to read input");
                
                if let Ok(id) = id_input.trim().parse::<usize>() {
                    todo_list.mark_done(id);
                } else {
                    println!("Invalid Task ID.");
                }
            }
            "4" => {
                println!("Enter Task ID to delete:");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Failed to read input");

                if let Ok(id) = id_input.trim().parse::<usize>() {
                    todo_list.delete_task(id);
                } else {
                    println!("Invalid Task ID.");
                }
            }
            "5" => {
                println!("Goodbye! 👋");
                break;
            }
            _ => println!("Invalid choice. Please enter a number between 1 and 5."),
        }
    }
}
