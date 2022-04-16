use std::{fmt, error};

#[derive(Debug, Clone)]
pub struct Error {
    err: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error for: {:?}", self.err)
    }
}

impl error::Error for Error {}

impl Error {
    pub fn new(err: String) -> Box<Error> {
        Box::new(Error {
            err
        })
    } 
}