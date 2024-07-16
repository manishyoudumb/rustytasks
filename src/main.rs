mod cli;
mod auth;
mod models;
mod error;
mod db;
mod commands;

use clap::Parser;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let cli = cli::Cli::parse();
    commands::execute_command(cli.command).await?;
    Ok(())
}