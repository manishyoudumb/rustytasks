mod cli;
mod commands;
mod db;
mod models;
mod error;
mod auth;
mod sync;

use clap::Parser;
use cli::Cli;
use commands::execute_command;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let cli = Cli::parse();
    execute_command(cli.command).await?;
    Ok(())
}
