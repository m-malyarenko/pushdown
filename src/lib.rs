pub mod error;
pub mod rules;

use std::cell::{Cell, RefCell};
use std::collections::HashSet;

use error::PdaError;

#[derive(Debug, Clone, Copy)]
pub enum PdaStateType {
    Accepting,
    NonAccepting,
}

pub type PdaState = char;

#[derive(Debug, Clone)]
pub struct PdaTransition {
    curr_state: PdaState,
    input: char,
    top: Option<char>,
    dest_state: PdaState,
    push_sequence: Option<Vec<char>>,
}

#[derive(Debug, Clone)]
pub struct PdaRules {
    pub input_symbols: HashSet<char>,
    pub stack_symbols: HashSet<char>,
    pub states: Vec<PdaState>,
    pub accepting_states: Vec<PdaState>,
    pub start_state: PdaState,
    pub transitions: Vec<PdaTransition>,
}

pub type PdaStatus = (PdaState, PdaStateType);

#[derive(Debug)]
pub struct Pda {
    rules: PdaRules,
    curr_state: Cell<PdaState>,
    stack: RefCell<Vec<char>>,
}

impl Pda {
    pub fn new(rules: PdaRules) -> Pda {
        let start_state = rules.start_state;

        Pda {
            rules,
            curr_state: Cell::new(start_state),
            stack: RefCell::new(Vec::new()),
        }
    }

    pub fn step(&self, input: char) -> Result<PdaStatus, PdaError> {
        if !self.rules.is_input_symbol(input) {
            return Err(PdaError::UnknownInputSymbol(input));
        }

        let curr_state = self.curr_state.get();
        let mut stack = self.stack.borrow_mut();
        let top = stack.first().copied();

        if let Some(transition) = self
            .rules
            .transitions
            .iter()
            .find(|&t| t.curr_state == curr_state && t.input == input && t.top == top)
        {
            self.curr_state.set(transition.dest_state);
            if let Some(push_sequence) = &transition.push_sequence {
                stack.extend(push_sequence)
            };
        } else {
            return Err(PdaError::UnresolvedTransition((curr_state, input, top)));
        };

        let new_curr_state = self.curr_state.get();

        Ok((new_curr_state, self.rules.get_state_type(new_curr_state)))
    }
}

impl PdaRules {
    fn is_input_symbol(&self, c: char) -> bool {
        self.input_symbols.contains(&c)
    }

    fn get_state_type(&self, s: PdaState) -> PdaStateType {
        if self.accepting_states.contains(&s) {
            PdaStateType::Accepting
        } else {
            PdaStateType::NonAccepting
        }
    }
}
