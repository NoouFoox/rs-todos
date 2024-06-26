mod cli;
mod database;
use clap::Parser;
use cli::{Cli, Commands};
use database::{Database, Record};

fn main() {
    let args = Cli::parse();
    let mut db = Database::open(".todos");
    match args.command {
        Commands::Info => println!("Rodo is a simple todo app."),
        Commands::Add { content } => {
            if let Some(content) = content {
                let id = db.read_record().last().map(|r| r.id + 1).unwrap_or(1);
                db.add_record(&Record { id, content });
            } else {
                println!("You need to specify the content of the todo item.")
            }
        }
        Commands::Remove { id } => {
            if id.is_none() {
                println!("You need to specify the id of the todo item.");
            } else {
                println!("Removing a todo item: {}", id.clone().unwrap());
                db.remove_record(id.unwrap().parse::<i32>().unwrap())
            }
        }
        Commands::List => {
            let todos = db.read_record();
            if todos.is_empty() {
                println!("No todos found!")
            } else {
                for todo in todos {
                    println!("{}:{}", todo.id, todo.content)
                }
            }
        }
    }
}
