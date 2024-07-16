pub(crate) use thiserror::Error;


#[derive(Error, Debug)]

pub enum TodoError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),

    

}
    