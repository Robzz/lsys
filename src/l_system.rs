//! This module contains the core of the L-system implementation.

use crate::action::Action;
use crate::rule::Rule;

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
/// Errors that an L-system can encounter.
pub enum Error {
  /// Attempt to pop an empty stack.
  PopEmptyStack
}

#[derive(Debug)]
/// Lindenmayer system.
/// TODO: the alphabet could be a string (easier for the user to create) that we then deduplicate
/// TODO: an easy potential optimization would be to figure out what tokens of the alphabet are terminals by subtracting
/// the tokens appearing in an action from the alphabet. Then, if the remaining string starts with a terminal, we can
/// move to the next as no rule should match.
pub struct LSystem {
  alphabet: HashSet<char>,
  axiom: String,
  rules: Vec<Rule>,
  // TODO: does this replace the alphabet ?
  actions: HashMap<char, Action>,
  state: String,
}

impl LSystem {
  /// Create a new `LSystem`.
  pub fn new(alphabet: HashSet<char>, axiom: String, rules: Vec<Rule>, actions: HashMap<char, Action>) -> LSystem {
    LSystem { alphabet, axiom: axiom.clone(), rules, actions, state: axiom }
  }

  /// Return the state of the L-system.
  pub fn state(&self) -> &str {
    &self.state
  }

  /// Perform an iteration on the L-system.
  pub fn step(&mut self) -> Result<(), Error> {
    let mut ptr: &str = self.state.as_ref();
    let mut new_state = String::new();
    loop {
      let mut jump = 1;
      let mut match_found = false;
      for rule in &self.rules {
        if let Some(m) = rule.matches(ptr) {
          // Perform the substitution
          new_state.push_str(m.rewrite());
          jump = m.match_len();
          match_found = true;
          break;
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