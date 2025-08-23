use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Does not contain split rule")]
    NoSplitter,
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("IO error")]
    Io,

    #[error("There was an error connecting to the file")]
    ConnectionError,

    #[error("There was an error with sql")]
    SqlError(#[from] sqlite::Error),

    #[error("There was an error creating the tables ")]
    TableError,
}
