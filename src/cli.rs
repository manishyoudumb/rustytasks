use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "todo")]

pub struct Cli {
    #[command(subcommand)] 
    pub command: Commands,
}

#[derive(Parser, Subcommand)]

pub enum Command {
    show {
        #[arg(short, long)]
        all: bool,
        #[arg(short, long)]
        completed: bool,
        #[arg(short, long)]
        incomplete: bool,
        list_name: Option<String>
    },

    Add {
        list_name: String,
        item: String,
    }
}