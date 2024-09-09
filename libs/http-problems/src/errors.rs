use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Reqwest error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("NotFoundError")]
    NotFoundError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
