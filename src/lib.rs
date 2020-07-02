//! This crate provides an implementation of [L-systems](https://en.wikipedia.org/wiki/L-system).
#![deny(missing_docs)]

mod action;
mod l_system;
mod rule;

pub use action::Action;
pub use l_system::LSystem;
pub use rule::{Rule, Match};