use crate::{
    action::Action,
    l_system::LSystem,
    rule::Rule
};

use thiserror::Error;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Error)]
/// Errors that can occur when building a L-system.
pub enum BuildError {
    #[error("no alphabet was supplied")]
    /// No alphabet supplied.
    NoAlphabet,
    #[error("no axiom was supplied")]
    /// No axiom supplied.
    NoAxiom
}

/// L-system builder.
pub struct Builder {
    actions: HashMap<char, Action>,
    alphabet: Option<HashSet<char>>,
    axiom: Option<String>,
    rules: Vec<Rule>,
}

impl Builder {
    /// Create a new `Builder`.
    pub fn new() -> Builder {
        Builder::default()
    }

    /// Build the L-system.
    pub fn build(self) -> Result<LSystem, BuildError> {
        let alphabet = self.alphabet.ok_or(BuildError::NoAlphabet)?;
        let non_terminals = self.actions.keys().copied().collect::<HashSet<_>>();
        let terminals = alphabet.difference(&non_terminals).copied().collect::<HashSet<_>>();
        let axiom = self.axiom.ok_or(BuildError::NoAxiom)?;
        Ok(LSystem::new(axiom, self.rules, self.actions, terminals))
    }

    /// Set the L-system actions.
    pub fn actions<I: IntoIterator<Item=(char, Action)>>(&mut self, actions: I) -> &mut Self {
        self.actions = actions.into_iter().collect();
        self
    }

    /// Set the L-system rules.
    pub fn rules<I: IntoIterator<Item=Rule>>(&mut self, rules: I) -> &mut Self {
       self.rules = rules.into_iter().collect();
       self
    }

    /// Set the L-system alphabet.
    pub fn alphabet<I: IntoIterator<Item=char>>(&mut self, alphabet: I) -> &mut Self {
       self.alphabet = Some(alphabet.into_iter().collect());
       self
    }

    /// Set the L-system axiom.
    pub fn axiom<S: AsRef<str>>(&mut self, axiom: S) -> &mut Self {
        self.axiom = Some(axiom.as_ref().to_owned());
        self
    }
}

impl Default for Builder {
    fn default() -> Builder {
        let actions = HashMap::new();
        let rules = Vec::new();
        let alphabet = None;
        let axiom = None;
        Builder { actions, alphabet, axiom, rules }
    }
}