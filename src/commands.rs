use std::io;

use crate::database::{Database, Record};

pub fn info() -> Result<(), io::Error> {
    println!("Rodo is a simple todo app.");
    Ok(())
}

pub fn add(db: &mut Database, content: Option<String>) -> Result<(), io::Error> {
    if let Some(content) = content {
        println!("Adding a todo item: {}", content);
        let id = db.read_records().last().map(|f| f.id + 1).unwrap_or(1);
        db.add_record(&Record { id, content })?;
        println!("Todo item added.");
        Ok(())
    } else {
        eprintln!("You need to specify the content of the todo item.");
        std::process::exit(1);
    }
}

pub fn remove(db: &mut Database, id: Option<String>) -> Result<(), io::Error> {
    if id.is_none() {
        println!("You need to specify the id of the todo item.");
        std::process::exit(1);
    }
    println!("Removing a todo item: {}", id.clone().unwrap());
    db.remove_record(id.unwrap().parse::<i32>().unwrap())?;
    println!("Todo item removed.");
    Ok(())
}

pub fn list(db:&mut Database) ->Result<(),io::Error>{
  let records = db.read_records();
  if records.is_empty(){
    println!("No todos found!");
    std::process::exit(1);
  }
  for record in records{
    println!("{}:{}",record.id,record.content);
  }
  Ok(())
}