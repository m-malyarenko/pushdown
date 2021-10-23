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

pub type PdaState = (char, PdaStateType);

#[derive(Debug, Clone, Copy)]
pub struct PdaTransitionLhs {
    curr_state: PdaState,
    input: char,
    top: Option<char>,
}

#[derive(Debug, Clone)]
pub struct PdaTransitionRhs {
    dest_state: PdaState,
    push: Option<Vec<char>>,
}

#[derive(Debug, Clone)]
pub struct PdaTransition {
    lhs: PdaTransitionLhs,
    rhs: PdaTransitionRhs,
}

#[derive(Debug, Clone)]
pub struct PdaRules {
    pub input_symbols: HashSet<char>,
    pub stack_symbols: HashSet<char>,
    pub states: Vec<PdaState>,
    pub start_state: PdaState,
    pub transitions: Vec<PdaTransition>,
}

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

    pub fn step(&self, input: char) -> Result<PdaState, PdaError> {
        if !self.rules.is_input_symbols(input) {
            return Err(PdaError::UnknownInputSymbol(input));
        }

        let curr_state = self.curr_state.get();
        let mut stack = self.stack.borrow_mut();
        let top = stack.first().copied();

        if let Some(transition) = self.rules.transitions.iter().find(|&t| {
            t.lhs.curr_state.0 == curr_state.0 && t.lhs.input == input && t.lhs.top == top
        }) {
            self.curr_state.set(transition.rhs.dest_state);
            if let Some(push_sequence) = &transition.rhs.push {
                stack.extend(push_sequence)
            };
        } else {
            return Err(PdaError::UnresolvedTransition((curr_state.0, input, top)));
        };

        Ok(self.curr_state.get())
    }
}

impl PdaRules {
    fn is_input_symbols(&self, c: char) -> bool {
        self.input_symbols.contains(&c)
    }
}
