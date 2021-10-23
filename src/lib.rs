pub mod error;
pub mod rules;

use std::collections::HashSet;
use std::cell::{Cell, RefCell};

use error::PdaError;

#[derive(Debug, Clone, Copy)]
pub enum PdaStateType {
    Accepting,
    NonAccepting,
}

pub type PdaState = (char, PdaStateType);

#[derive(Debug)]
pub struct PdaTransition {
    curr_state: PdaState,
    input: char,
    top: char,
    dest_state: PdaState,
    push: Option<Vec<char>>,
}

#[derive(Debug)]
pub struct PdaRules<'a> {
    pub input_symbols: HashSet<char>,
    pub stack_symbols: HashSet<char>,
    pub states: Vec<PdaState>,
    pub start_state: &'a PdaState,
    pub transitions: Vec<PdaTransition>,
}

#[derive(Debug)]
pub struct Pda<'a> {
    rules: PdaRules<'a>,
    current_state: Cell<PdaState>,
    stack: RefCell<Vec<char>>,
}

impl<'a> Pda<'a> {
    pub fn new(rules: &PdaRules) -> Pda<'a> {
        unimplemented!()
    }

    pub fn step(&self, input: char) -> Result<PdaState, PdaError> {
        unimplemented!()
    }

    pub fn get_top(&self) -> Option<char> {
        unimplemented!()
    }
}
