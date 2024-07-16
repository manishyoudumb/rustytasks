use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
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
        list_name: Option<String>
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

    Login,
    Logout,
    Push,
    Pull,
}


#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_parsing_complex() {
        // Test Show command with multiple flags
        let show_args = vec!["todo", "show", "--all", "--completed", "--incomplete", "my_list"];
        let cli = Cli::parse_from(show_args);
        match cli.command {
            Command::Show { all, completed, incomplete, list_name } => {
                assert!(all);
                assert!(completed);
                assert!(incomplete);
                assert_eq!(list_name, Some("my_list".to_string()));
            }
            _ => panic!("Expected Show command"),
        }

        // Test Show command without list name
        let show_args = vec!["todo", "show", "--completed"];
        let cli = Cli::parse_from(show_args);
        match cli.command {
            Command::Show { all, completed, incomplete, list_name } => {
                assert!(!all);
                assert!(completed);
                assert!(!incomplete);
                assert_eq!(list_name, None);
            }
            _ => panic!("Expected Show command"),
        }

        // Test Add command with spaces in item description
        let add_args = vec!["todo", "add", "shopping list", "buy milk and eggs"];
        let cli = Cli::parse_from(add_args);
        match cli.command {
            Command::Add { list_name, item } => {
                assert_eq!(list_name, "shopping list");
                assert_eq!(item, "buy milk and eggs");
            }
            _ => panic!("Expected Add command"),
        }

        // Test Complete command with large item number
        let complete_args = vec!["todo", "complete", "work tasks", "9999"];
        let cli = Cli::parse_from(complete_args);
        match cli.command {
            Command::Complete { list_name, item_number } => {
                assert_eq!(list_name, "work tasks");
                assert_eq!(item_number, 9999);
            }
            _ => panic!("Expected Complete command"),
        }

        // Test Incomplete command
        let incomplete_args = vec!["todo", "incomplete", "personal", "5"];
        let cli = Cli::parse_from(incomplete_args);
        match cli.command {
            Command::Incomplete { list_name, item_number } => {
                assert_eq!(list_name, "personal");
                assert_eq!(item_number, 5);
            }
            _ => panic!("Expected Incomplete command"),
        }

        // Test Remove command with both list and item number
        let remove_args = vec!["todo", "remove", "project", "3"];
        let cli = Cli::parse_from(remove_args);
        match cli.command {
            Command::Remove { list_name, item_number } => {
                assert_eq!(list_name, Some("project".to_string()));
                assert_eq!(item_number, Some(3));
            }
            _ => panic!("Expected Remove command"),
        }

        // Test Remove command with only list name
        let remove_args = vec!["todo", "remove", "old_list"];
        let cli = Cli::parse_from(remove_args);
        match cli.command {
            Command::Remove { list_name, item_number } => {
                assert_eq!(list_name, Some("old_list".to_string()));
                assert_eq!(item_number, None);
            }
            _ => panic!("Expected Remove command"),
        }

        // Test Login, Logout, Push, and Pull commands
        let login_args = vec!["todo", "login"];
        let logout_args = vec!["todo", "logout"];
        let push_args = vec!["todo", "push"];
        let pull_args = vec!["todo", "pull"];

        assert!(matches!(Cli::parse_from(login_args).command, Command::Login));
        assert!(matches!(Cli::parse_from(logout_args).command, Command::Logout));
        assert!(matches!(Cli::parse_from(push_args).command, Command::Push));
        assert!(matches!(Cli::parse_from(pull_args).command, Command::Pull));
    }
}
