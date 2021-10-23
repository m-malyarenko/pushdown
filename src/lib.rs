pub mod error;
pub mod rules;

use std::cell::{Cell, RefCell};

use error::PdaError;
use rules::PdaRules;

pub type PdaState = char;

#[derive(Debug, Clone, Copy)]
pub enum PdaStateType {
    Accepting,
    NonAccepting,
}

pub type PdaStatus = (PdaState, PdaStateType);

#[derive(Debug, Clone)]
struct PdaTransition {
    curr_state: PdaState,
    input: char,
    top: Option<char>,
    dest_state: PdaState,
    push_sequence: Option<Vec<char>>,
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
            .find(|t| t.curr_state == curr_state && t.input == input && t.top == top)
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
