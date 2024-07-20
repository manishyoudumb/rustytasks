use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Item not found: {0}")]
    ItemNotFound(String),

    #[error("List not found: {0}")]
    ListNotFound(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("MongoDB error: {0}")]
    MongoError(#[from] mongodb::error::Error),
}

pub type TodoResult<T> = Result<T, TodoError>;


#[test]
fn test_todo_error_messages() {
    let config_error = TodoError::ConfigError("Oops".to_string());
    assert_eq!(format!("{}", config_error), "Configuration error: Oops");
    // Because who doesn't love a good config error, am I right?
}
