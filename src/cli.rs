use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
pub struct Cli {
    #[command(subcommand)] 
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Show {
        #[arg(short, long)]
        all: bool,
        #[arg(short, long)]
        completed: bool,
        #[arg(short, long)]
        incomplete: bool,
        list_name: Option<String>,
    },
    Add {
        list_name: String,
        item: String,
    },
    Complete {
        list_name: String,
        item_number: usize,
    },
    Incomplete {
        list_name: String,
        item_number: usize,
    },
    Remove {
        list_name: Option<String>,
        item_number: Option<usize>,
    },
    Push,
    Pull,
    Login,
    Logout,
}
