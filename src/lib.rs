pub mod error;
mod rules;

use std::cell::{Cell, RefCell};
use std::collections::HashSet;

use error::PdaError;

#[derive(Debug, Clone, Copy)]
pub enum PdaStateType {
    Accepting,
    NonAccepting,
}

pub type PdaState = (char, PdaStateType);

#[derive(Debug)]
struct PdaTransition {
    curr_state: PdaState,
    input: char,
    top: char,
    dest_state: PdaState,
    push: Option<Vec<char>>,
}

#[derive(Debug)]
pub struct PdaRules {
    input_symbols: HashSet<char>,
    stack_symbols: HashSet<char>,
    states: Vec<PdaState>,
    start_state: PdaState,
    transitions: Vec<PdaTransition>,
}

#[derive(Debug)]
pub struct Pda {
    rules: PdaRules,
    current_state: Cell<PdaState>,
    stack: RefCell<Vec<char>>,
}

impl Pda {
    pub fn new(rules: PdaRules) -> Pda {
        unimplemented!()
    }

    pub fn step(&self, input: char) -> Result<PdaState, PdaError> {
        unimplemented!()
    }

    pub fn get_top(&self) -> Option<char> {
        unimplemented!()
    }
}
