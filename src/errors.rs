use thiserror::Error;

#[derive(Error, Debug)]
pub enum APIError {
    #[error("unknown data store error")]
    Unknown,
}

