pub(crate) use thiserror::Error;


#[derive(Error, Debug)]

pub enum TodoError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Item not found: {0}")]
    ItemNotFound(String),

    #[error("List not found: {0}")]
    ListNotFound(String),

}
    
pub type TodoResult<T> = Result<T, TodoError>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_todo_error_variants() {
        let db_error = TodoError::DatabaseError(mongodb::error::Error::from(io::Error::new(io::ErrorKind::Other, "DB error")));
        assert!(matches!(db_error, TodoError::DatabaseError(_)));

        let io_error = TodoError::IoError(io::Error::new(io::ErrorKind::NotFound, "File not found"));
        assert!(matches!(io_error, TodoError::IoError(_)));

        let invalid_input = TodoError::InvalidInput("Invalid user input".to_string());
        assert!(matches!(invalid_input, TodoError::InvalidInput(_)));

        let item_not_found = TodoError::ItemNotFound("Item 1 in list 'Todo'".to_string());
        assert!(matches!(item_not_found, TodoError::ItemNotFound(_)));

        let list_not_found = TodoError::ListNotFound("Shopping list".to_string());
        assert!(matches!(list_not_found, TodoError::ListNotFound(_)));
    }

    #[test]
    fn test_error_messages() {
        let db_error = TodoError::DatabaseError(mongodb::error::Error::from(io::Error::new(io::ErrorKind::Other, "DB error")));
        assert!(db_error.to_string().contains("Database error"));

        let io_error = TodoError::IoError(io::Error::new(io::ErrorKind::NotFound, "File not found"));
        assert!(io_error.to_string().contains("IO error"));

        let invalid_input = TodoError::InvalidInput("Invalid user input".to_string());
        assert_eq!(invalid_input.to_string(), "Invalid input: Invalid user input");

        let item_not_found = TodoError::ItemNotFound("Item 1 in list 'Todo'".to_string());
        assert_eq!(item_not_found.to_string(), "Item not found: Item 1 in list 'Todo'");

        let list_not_found = TodoError::ListNotFound("Shopping list".to_string());
        assert_eq!(list_not_found.to_string(), "List not found: Shopping list");
    }
}

#[cfg(test)]
#[cfg(test)]
mod integration_tests {
    use clap::Parser as _;

    use crate::{cli, commands};

    use std::env;

    #[tokio::test]
    async fn test_main_function() {
        // Set up test environment
        env::set_var("MONGODB_URI", "mongodb://localhost:27017");
        env::set_var("GOOGLE_CLIENT_ID", "test_client_id");
        env::set_var("GOOGLE_CLIENT_SECRET", "test_client_secret");

        // Create a mock CLI command
        let args = vec!["todo", "show", "--all"];
        let cli = cli::Cli::parse_from(args);

        // Execute the command
        let result = commands::execute_command(cli.command).await;

        // Assert that the command execution was successful
        assert!(result.is_ok());
    }
}
