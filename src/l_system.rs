//! This module contains the core of the L-system implementation.

use crate::{
    action::Action,
    builder::Builder,
    rule::Rule
};

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
/// Errors that an L-system can encounter.
pub enum Error {
  /// Attempt to pop an empty stack.
  PopEmptyStack
}

#[derive(Debug)]
/// Lindenmayer system.
pub struct LSystem {
    state: String,
    rules: Vec<Rule>,
    actions: HashMap<char, Action>,
    terminals: HashSet<char>
}

impl LSystem {
    /// Create a new `LSystem`.
    pub (crate) fn new(axiom: String, rules: Vec<Rule>, actions: HashMap<char, Action>, terminals: HashSet<char>) -> LSystem {
        LSystem { state: axiom, rules, actions, terminals }
    }

    /// Return a L-system `Builder`.
    pub fn builder() -> Builder {
        Builder::default()
    }

    /// Return the state of the L-system.
    pub fn state(&self) -> &str {
        &self.state
    }

    /// Return the actions handled by the L-system.
    pub fn actions(&self) -> &HashMap<char, Action> {
        &self.actions
    }

    /// Perform an iteration on the L-system.
    pub fn step(&mut self) -> Result<(), Error> {
        let mut ptr: &str = self.state.as_ref();
        let mut new_state = String::new();
        loop {
            let mut jump = 1;
            let mut match_found = false;
            if !self.terminals.contains(&ptr.chars().next().unwrap()) {
                for rule in &self.rules {
                    if let Some(m) = rule.matches(ptr) {
                        // Perform the substitution
                        new_state.push_str(m.rewrite());
                        jump = m.match_len();
                        match_found = true;
                        break;
                    }
                }
            }
            // No match was found, append the first character and try with the next one
            if !match_found {
                let c = ptr.chars().next().unwrap();
                jump = c.len_utf8();
                new_state.push(c);
            }
            if ptr.len() > 1 {
                ptr = ptr.get(jump..).unwrap()
            }
            else {
                break;
            }
        }
        self.state = new_state;
        
        Ok(())
    }
}