use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PdaError {
    UnknownInputSymbol(char),
    UnresolvedTransition((char, char, Option<char>)),
}

impl fmt::Display for PdaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_type_name = "pda error";

        match self {
            PdaError::UnknownInputSymbol(c) => {
                write!(f, "{}: unknown input symbol '{}'", error_type_name, c)
            }
            PdaError::UnresolvedTransition((state, input, top)) => {
                let top = if top.is_some() { top.unwrap() } else { 'ε' };

                write!(
                    f,
                    "{}: unresolved transition (state: '{}', input: '{}', top: '{}') -> ?",
                    error_type_name, state, input, top
                )
            }
        }
    }
}

impl Error for PdaError {}

#[derive(Debug)]
pub enum PdaRulesError {}

impl fmt::Display for PdaRulesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl Error for PdaRulesError {}
