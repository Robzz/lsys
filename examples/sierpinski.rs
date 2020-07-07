use lsys::{Action, ImageRenderer, LSystem, Renderer, Rule};

use std::{
    collections::HashMap,
    f64::consts::PI
};

const STEP_DISTANCE: f64 = 20.;
const STEP_ANGLE: f64 = 2. * PI / 3.;

fn main() -> () {
    let axiom = "F-G-G".to_owned();
    let mut rules = Vec::new();
    let mut actions = HashMap::new();
    rules.push(Rule::new("F".to_owned(), "F−G+F+G−F".to_owned()));
    rules.push(Rule::new("G".to_owned(), "GG".to_owned()));
    actions.insert('F', Action::Forward(STEP_DISTANCE));
    actions.insert('G', Action::Forward(STEP_DISTANCE));
    actions.insert('+', Action::Rotate(-STEP_ANGLE));
    actions.insert('-', Action::Rotate(STEP_ANGLE));
  
    let mut builder = LSystem::builder();
    builder.alphabet("FG+-".chars())
        .actions(actions)
        .rules(rules)
        .axiom(axiom);
    let mut system = builder.build().unwrap();

    for i in 0..5 {
        system.step().unwrap();
        println!("System after iteration {}: {}", i, system.state());
        let renderer = ImageRenderer::new();
        let img = renderer.render(&system);
        img.save(format!("img_iter_{}.png", i)).unwrap();
    }
}