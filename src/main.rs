mod cli;
mod commands;
mod database;
mod utils;
use clap::Parser;
use cli::{
    Cli,
    Commands::{Add, Info, List, Remove},
};
use database::Database;

fn main() {
    let args = Cli::parse();
    let mut db = Database::open();
    let result = match args.command {
        Info => commands::info(),
        Add { content } => commands::add(&mut db, content),
        Remove { id } => commands::remove(&mut db, id),
        List => commands::list(&mut db),
    };
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
