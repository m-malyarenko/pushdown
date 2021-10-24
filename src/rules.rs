use serde_json::Value as JsonValue;
use std::collections::HashSet;

use super::{PdaState, PdaStateType, PdaTransition};
use crate::error::PdaRulesError;

#[derive(Debug, Clone)]
pub struct PdaRules {
    input_symbols: HashSet<char>,
    stack_symbols: HashSet<char>,
    states: HashSet<PdaState>,
    accept_states: HashSet<PdaState>,
    pub(super) start_state: PdaState,
    pub(super) transitions: Vec<PdaTransition>,
}

impl PdaRules {
    pub fn from_json(json_str: &str) -> Result<PdaRules, PdaRulesError> {
        if let Ok(json_values) = serde_json::from_str(json_str) {
            let json_values: JsonValue = json_values;

            /* Input symbols */
            let input_symbols: HashSet<char> = json_values["input_symbols"]
                .as_str()
                .expect("invalid input symbols definition format")
                .chars()
                .collect();

            if input_symbols.is_empty() {
                return Err(PdaRulesError::InvalidRule(
                    "empty input symbols set".to_string(),
                ));
            }

            /* Stack symbols */
            let stack_symbols: HashSet<char> = json_values["stack_symbols"]
                .as_str()
                .expect("invalid stack symbols definition format")
                .chars()
                .collect();

            if stack_symbols.is_empty() {
                return Err(PdaRulesError::InvalidRule(
                    "empty stack symbols set".to_string(),
                ));
            }

            /* States and Accepting states */
            let raw_states: Vec<(PdaState, bool)> = json_values["states"]
                .as_array()
                .expect("invalis states definition format")
                .iter()
                .map(|state| {
                    let symbol: &str = state["symbol"]
                        .as_str()
                        .expect("invalid state symbol definition format");
                    let accepting: bool = state["accepting"]
                        .as_bool()
                        .expect("invalid state symbol definition format");
                    if symbol.len() != 1 {
                        panic!("invalid state symbol format");
                    }

                    (symbol.chars().nth(0).unwrap(), accepting)
                })
                .collect();

            for (i, &state) in raw_states.iter().enumerate() {
                if raw_states.iter().skip(i + 1).any(|&(s, _)| s == state.0) {
                    return Err(PdaRulesError::RulesParserFailed(format!(
                        "'{}' state redefinition",
                        state.0
                    )));
                }
            }

            let states: HashSet<PdaState> = raw_states.iter().map(|&(c, _)| c).collect();

            let accept_states: HashSet<PdaState> = raw_states
                .iter()
                .filter(|&(_, b)| *b)
                .map(|&(c, _)| c)
                .collect();

            /* Start state */
            let start_state_str = json_values["start_state"]
                .as_str()
                .expect("invalid start state definition format");

            if start_state_str.len() != 1 {
                panic!("invalid start state symbol")
            };

            let start_state: PdaState = start_state_str.chars().nth(0).unwrap();

            if !states.contains(&start_state) {
                return Err(PdaRulesError::InvalidRule(
                    "unknown start state".to_string(),
                ));
            };

            /* Transitions */
            let transitions: Vec<PdaTransition> = json_values["transitions"]
                .as_array()
                .expect("invalid transinions definition format")
                .iter()
                .map(|transition| {
                    /* Parse transition fields */
                    let curr_state = transition["curr_state"].as_str().expect(
                        format!("invalid curr state in transition {}", transition).as_str(),
                    );
                    let input = transition["input"]
                        .as_str()
                        .expect(format!("invalid input in transition {}", transition).as_str());
                    let top = if transition["top"].is_string() {
                        Some(transition["top"].as_str().unwrap())
                    } else if transition["top"].is_null() {
                        None
                    } else {
                        panic!("invalid stack top in transition {}", transition);
                    };
                    let dest_state = transition["dest_state"].as_str().expect(
                        format!("invalid dest state in transition {}", transition).as_str(),
                    );
                    let push_sequence = if transition["push_sequence"].is_string() {
                        Some(transition["push_sequence"].as_str().unwrap())
                    } else if transition["push_sequence"].is_null() {
                        None
                    } else {
                        panic!("invalid push sequence in transition {}", transition)
                    };

                    /* Check transition fields */
                    if curr_state.len() != 1
                        || !states.contains(&curr_state.chars().nth(0).unwrap())
                    {
                        panic!("unknown curr state in transition {}", transition);
                    }
                    if input.len() != 1 || !input_symbols.contains(&input.chars().nth(0).unwrap()) {
                        panic!("unknown input in transition {}", transition);
                    }
                    if let Some(t) = top {
                        if t.len() != 1 || !stack_symbols.contains(&t.chars().nth(0).unwrap()) {
                            panic!("unknown stack top in transition {}", transition)
                        }
                    };
                    if dest_state.len() != 1
                        || !states.contains(&dest_state.chars().nth(0).unwrap())
                    {
                        panic!("unknown dest state in transition {}", transition);
                    };
                    if let Some(p) = push_sequence {
                        if p.chars().any(|c| !stack_symbols.contains(&c)) {
                            panic!(
                                "unknown stack symbol in push sequence in transition {}",
                                transition
                            )
                        }
                    };

                    let curr_state = curr_state.chars().nth(0).unwrap();
                    let input = input.chars().nth(0).unwrap();
                    let top = if let Some(t) = top {
                        Some(t.chars().nth(0).unwrap())
                    } else {
                        None
                    };
                    let dest_state = dest_state.chars().nth(0).unwrap();
                    let push_sequence = if let Some(p) = push_sequence {
                        Some(p.chars().collect())
                    } else {
                        None
                    };

                    PdaTransition {
                        curr_state,
                        input,
                        top,
                        dest_state,
                        push_sequence,
                    }
                })
                .collect();

            Ok(PdaRules {
                input_symbols,
                stack_symbols,
                states,
                accept_states,
                start_state,
                transitions,
            })
        } else {
            return Err(PdaRulesError::RulesParserFailed("JSON error".to_string()));
        }
    }

    pub(super) fn is_input_symbol(&self, c: char) -> bool {
        self.input_symbols.contains(&c)
    }

    pub(super) fn get_state_type(&self, s: PdaState) -> PdaStateType {
        if self.accept_states.contains(&s) {
            PdaStateType::Accepting
        } else {
            PdaStateType::NonAccepting
        }
    }
}

#[test]
fn parsing_test() {
    let json_script =
        std::fs::read_to_string("D:/Workspace/VSCode/Rust/Projects/pushdown/resources/ab.json")
            .expect("failed to read a file");

    let rules = PdaRules::from_json(&json_script);
    println!("rules {:?}", rules);
}
