#![allow(clippy::upper_case_acronyms)]
#[derive(thiserror::Error, Debug)]
pub enum MailError {
    #[error("{0} is not a recipient")]
    InvalidAddress(String),

    #[error(transparent)]
    MessageCreation(#[from] lettre::error::Error),

    #[error(transparent)]
    SMTP(#[from] lettre::transport::smtp::Error),
}
