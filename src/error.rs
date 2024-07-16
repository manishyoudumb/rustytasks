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