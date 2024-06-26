use clap::{Parser, Subcommand};

#[derive(Parser)]
// #[clap(version, about)] 会输出我们在 Cargo.toml 中定义的 version 和 about 字段。
#[clap(version, about)]
// #[clap(propagate_version = true)] 会将 version 信息传递给子命令。
#[clap(propagate_version = true)]
pub struct Cli {
  // #[clap(subcommand)] 用于定义一个子命令。 
    #[clap(subcommand)]
    pub command: Commands,
}
#[derive(Debug, Subcommand)]
pub enum Commands {
  // #[clap(about = "xxx")] 会输出命令的相关说明。
    #[clap(about = "Show rodo info.")]
    Info,
    #[clap(about = "Add a new todo.")]
    Add {
      // #[clap(help = "xxx")] 帮助信息
        #[clap(help = "The content of the todo.")]
        content: Option<String>,
    },
    #[clap(about = "Remove a todo item.")]
    // #[clap(visible_aliases = & ["xxx"])] 会为命令添加别名。
    #[clap(visible_alias = "rm")]
    Remove {
        #[clap(help = "The item id to remove.")]
        id: Option<String>,
    },
    #[clap(about = "List all todo items.")]
    #[clap(visible_aliases = & ["ls", "ll","la"])]
    List,
}
