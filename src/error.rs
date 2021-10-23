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

#[derive(Debug)]
pub enum PdaRulesError {

}

impl fmt::Display for PdaRulesError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl Error for PdaRulesError {}