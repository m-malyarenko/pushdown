use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum PdaError {

}

impl fmt::Display for PdaError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl Error for PdaError {}