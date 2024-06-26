mod database;

use std::env;

use database::Database;

fn main() {
    // env::args() 返回一个迭代器，其中包含程序的所有命令行参数
    let args = env::args().collect::<Vec<String>>();
    // 如果没有参数，则打印使用说明
    if args.len() < 2 {
        println!("Usage: rodo [add|rm|ls] [args]");
        return;
    }
    // 获取第一个参数 作为命令
    let command = &args[1];
    // 打开一个名为 .todos 的模拟数据库
    let mut db = Database::open(".todos");

    // as_str 转换成字符串切片
    match command.as_str() {
        // 如果命令是 add 则添加一个新的 todo
        "add" => {
            // 如果没有提供要添加的 todo，则打印使用说明
            if args.len() < 3 {
                println!("Usage: rodo add [todo]");
                return;
            }
            // 将所有参数连接成一个字符串 作为 todo 的内容 使用 join 方法
            let content = &args[2..].join(" ");
            // 生成一个新的 id
            let id = db.read_record().last().map(|r| r.id + 1).unwrap_or(1);
            // 将 todo 添加到数据库
            db.add_record(&database::Record {
                id,
                content: content.to_string(),
            });
        }
        // 如果命令是 ls 则列出所有的 todos
        "ls" => {
            // 读取所有的 todos
            let todos = db.read_record();
            // 如果没有 todos 则打印提示信息
            if todos.is_empty() {
                println!("No todos found!")
            } else {
                // 否则打印所有的 todos
                for todo in todos {
                    println!("{}:{}", todo.id, todo.content);
                }
            }
        }
        // 如果命令是 rm 则删除一个 todo
        "rm" => {
            // 如果没有提供要删除的 todo 的 id，则打印使用说明
            if args.len() < 3 {
                println!("Usage: rodo rm [id]");
                return;
            }else{
                // 将 id 解析为整数
                let id = args[2].parse::<i32>().unwrap();
                db.remove_record(id);
            }
        }
        // 如果命令是未知的，则打印错误信息
        _ => println!("Unknown command: {}", command),
    }
}
