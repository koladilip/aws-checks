use std::{
    error::Error,
    fmt::{self, Display},
};

pub mod credentials;

#[derive(Debug, Clone)]
pub struct AWSError {
    message: String,
}

impl AWSError {
    pub fn new<T: Display>(err: T) -> Self {
        AWSError {
            message: err.to_string(),
        }
    }
    /// Get a reference to the awserror's message.
    pub fn message(&self) -> &String {
        &self.message
    }
}

impl fmt::Display for AWSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AWS Error: {}", self.message)
    }
}

impl Error for AWSError {}