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

    pub fn reset(&self) {
        self.curr_state.set(self.rules.start_state);
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

            stack.pop();

            if let Some(push_sequence) = &transition.push_sequence {
                stack.extend(push_sequence.iter().rev())
            };
        } else {
            return Err(PdaError::UnresolvedTransition((curr_state, input, top)));
        };

        let new_curr_state = self.curr_state.get();

        Ok((new_curr_state, self.rules.get_state_type(new_curr_state)))
    }

    pub fn get_curr_state(&self) -> PdaStatus {
        (self.curr_state.get(), self.rules.get_state_type(self.curr_state.get()))
    }
}

#[test]
fn pushdown_test() {
    let json_script =
    std::fs::read_to_string("D:/Workspace/VSCode/Rust/Projects/pushdown/resources/ab.json")
        .expect("failed to read a file");
    let rules = PdaRules::from_json(&json_script).unwrap();
    let pda = Pda::new(rules);

    pda.reset();
    let input_string = "ab#";
    for input in input_string.chars() {
        pda.step(input).unwrap();
    }
    assert_eq!(pda.get_curr_state().0, 'A');
    assert!(matches!(pda.get_curr_state().1, PdaStateType::Accepting));

    pda.reset();
    let input_string = "abb#";
    for input in input_string.chars() {
        pda.step(input).unwrap();
    }
    assert_eq!(pda.get_curr_state().0, 'E');
    assert!(matches!(pda.get_curr_state(), ('E', PdaStateType::NonAccepting)));

    pda.reset();
    let input_string = "#";
    for input in input_string.chars() {
        pda.step(input).unwrap();
    }
    assert_eq!(pda.get_curr_state().0, 'A');
    assert!(matches!(pda.get_curr_state().1, PdaStateType::Accepting));

    pda.reset();
    let input_string = "aabababb#";
    for input in input_string.chars() {
        pda.step(input).unwrap();
    }
    assert_eq!(pda.get_curr_state().0, 'A');
    assert!(matches!(pda.get_curr_state().1, PdaStateType::Accepting));

    pda.reset();
    let input_string = "aabbbababb#";
    for input in input_string.chars() {
        pda.step(input).unwrap();
    }
    assert_eq!(pda.get_curr_state().0, 'E');
    assert!(matches!(pda.get_curr_state().1, PdaStateType::NonAccepting));
}
