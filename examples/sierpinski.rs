use lsys::{Action, LSystem, Rule};

use std::collections::{HashMap, HashSet};

const STEP_DISTANCE: f64 = 20.;
const STEP_ANGLE: f64 = 120.;

fn main() -> () {
  let axiom = "F-G-G".to_owned();
  let mut alphabet = HashSet::new();
  let mut rules = Vec::new();
  let mut actions = HashMap::new();
  alphabet.insert('F');
  alphabet.insert('F');
  alphabet.insert('+');
  alphabet.insert('-');
  rules.push(Rule::new("F".to_owned(), "F−G+F+G−F".to_owned()));
  rules.push(Rule::new("G".to_owned(), "GG".to_owned()));
  actions.insert('F', Action::Forward(STEP_DISTANCE));
  actions.insert('G', Action::Forward(STEP_DISTANCE));
  actions.insert('+', Action::Rotate(-STEP_ANGLE));
  actions.insert('-', Action::Rotate(STEP_ANGLE));
  
  let mut system = LSystem::new(alphabet, axiom, rules, actions);

  for i in 0..5 {
    system.step().unwrap();
    println!("System after iteration {}: {}", i, system.state());
  }
}