use std::collections::HashSet;

use super::{PdaState, PdaStateType, PdaTransition};

#[derive(Debug, Clone)]
pub struct PdaRules {
    input_symbols: HashSet<char>,
    stack_symbols: HashSet<char>,
    states: Vec<PdaState>,
    accepting_states: Vec<PdaState>,
    pub(super) start_state: PdaState,
    pub(super) transitions: Vec<PdaTransition>,
}

impl PdaRules {
    pub(super) fn is_input_symbol(&self, c: char) -> bool {
        self.input_symbols.contains(&c)
    }

    pub(super) fn get_state_type(&self, s: PdaState) -> PdaStateType {
        if self.accepting_states.contains(&s) {
            PdaStateType::Accepting
        } else {
            PdaStateType::NonAccepting
        }
    }
}