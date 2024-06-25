mod database;

use std::env;

use database::Database;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: rodo [add|rm|ls] [args]");
        return;
    }
    let command = &args[1];
    let mut db = Database::open(".todos");

    // as_str 转换成字符串切片
    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Usage: rodo add [todo]");
                return;
            }
            let content = &args[2..].join(" ");
            let id = db.read_record().last().map(|r| r.id + 1).unwrap_or(1);
            db.add_record(&database::Record {
                id,
                content: content.to_string(),
            });
        }
        "ls" => {
            let todos = db.read_record();
            if todos.is_empty() {
                println!("No todos found!")
            } else {
                for todo in todos {
                    println!("{}: {}", todo.id, todo.content);
                }
            }
        }
        "rm" => {
            if args.len() < 3 {
                println!("Usage: rodo rm [id]");
                return;
            }else{
                let id = args[2].parse::<i32>().unwrap();
                db.remove_record(id);
            }
        }
        _ => println!("Unknown command: {}", command),
    }
}
