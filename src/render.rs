use crate::{Action, LSystem};

use image::{Rgb, RgbImage};
use imageproc::{
    drawing::draw_antialiased_line_segment_mut,
    pixelops::interpolate
};
use nalgebra::{Vector2, Point2};

use std::f64::consts::FRAC_PI_2;

/// Trait implemented by types able to render an L-system.
pub trait Renderer {
    /// Output type of the renderer.
    type Output;

    /// Render the given L-system.
    fn render(&self, system: LSystem) -> Self::Output;
}

/// Renders L-systems to color images.
#[derive(Debug, Default)]
pub struct ImageRenderer {
}

impl Renderer for ImageRenderer {
    type Output = RgbImage;

    fn render(&self, system: LSystem) -> RgbImage {
        let mut img = RgbImage::new(1280, 1024);
        let actions = system.actions();
        let mut turtle = Turtle::new();
        turtle.set_position(Point2::new(640., 1000.));
        turtle.set_orientation(FRAC_PI_2);

        for c in system.state().chars() {
            match actions.get(&c) {
                Some(Action::Forward(distance)) => {
                    let p_start = turtle.position();
                    turtle.forward(*distance);
                    let p_end = turtle.position();
                    Self::draw_line(p_start, p_end, &mut img);
                },
                Some(Action::Backwards(distance)) => {
                    let p_start = turtle.position();
                    turtle.backwards(*distance);
                    let p_end = turtle.position();
                    Self::draw_line(p_start, p_end, &mut img);
                },
                Some(Action::Rotate(angle)) => {
                    turtle.rotate(*angle);
                },
                Some(Action::StackPush) => {
                    turtle.stack_push()
                },
                Some(Action::StackPop) => {
                    turtle.stack_pop()
                },
                None => { }
            }
        }

        img
    }
}

impl ImageRenderer {
    /// Construct a new `ImageRenderer`.
    pub fn new() -> ImageRenderer {
        ImageRenderer::default()
    }

    fn draw_line(p_start: Point2<f64>, p_end: Point2<f64>, buffer: &mut RgbImage) {
        draw_antialiased_line_segment_mut(
            buffer,
            (p_start[0] as i32, p_start[1] as i32),
            (p_end[0] as i32, p_end[1] as i32),
            Rgb([255, 255, 255]),
            interpolate
        );
    }
}

#[derive(Debug)]
pub struct Turtle {
    state: TurtleState,
    stack: Vec<TurtleState>
}

impl Turtle {
    /// Create a new `Turtle`.
    fn new() -> Turtle {
        Turtle { state: TurtleState::default(), stack: Vec::new() }
    }

    /// Return the turtle position.
    pub fn position(&self) -> Point2<f64> {
        self.state.position()
    }

    /// Set the turtle position.
    pub fn set_position(&mut self, position: Point2<f64>) {
        self.state.set_position(position);
    }

    /// Move the turtle forward by the specified distance.
    pub fn forward(&mut self, distance: f64) {
        self.state.forward(distance);
    }

    /// Move the turtle backwards by the specified distance.
    pub fn backwards(&mut self, distance: f64) {
        self.state.backwards(distance);
    }

    /// Set the turtle orientation
    pub fn set_orientation(&mut self, orientation: f64) {
        self.state.set_orientation(orientation);
    }

    /// Rotate the turtle by the specified angle, in radians. A positive angle means a rotation in the counterclockwise direction.
    pub fn rotate(&mut self, angle: f64) {
        self.state.rotate(angle);
    }

    /// Push the current turtle state on the stack.
    pub fn stack_push(&mut self) {
        self.stack.push(self.state.clone());
    }

    /// Restore the state currently on top of the stack.
    pub fn stack_pop(&mut self) {
        self.state = self.stack.pop().expect("Cannot pop from an empty stack");
    }
}

#[derive(Debug, Clone)]
pub struct TurtleState {
    position: Point2<f64>,
    orientation: f64
}

impl Default for TurtleState {
    fn default() -> TurtleState {
        TurtleState { position: Point2::origin(), orientation: 0. }
    }
}

impl TurtleState {
    /// Create a new `TurtleState`.
    pub fn new(position: Point2<f64>, orientation: f64) -> TurtleState {
        TurtleState { position, orientation }
    }

    /// Return the current turtle position.
    pub fn position(&self) -> Point2<f64> {
        self.position
    }

    /// Set the turtle position.
    pub fn set_position(&mut self, position: Point2<f64>) {
        self.position = position
    }

    /// Return the current turtle direction vector.
    pub fn direction(&self) -> Vector2<f64> {
        Vector2::new(self.orientation.cos(), self.orientation.sin())
    }

    /// Rotate the turtle by the given angle.
    pub fn rotate(&mut self, angle: f64) {
        self.orientation += angle;
    }

    /// Set the turtle orientation.
    pub fn set_orientation(&mut self, orientation: f64) {
        self.orientation = orientation;
    }

    /// Move the turtle forward by the specified distance.
    pub fn forward(&mut self, distance: f64) {
        self.position += self.direction() * distance;
    }

    /// Move the turtle backwards by the specified distance.
    pub fn backwards(&mut self, distance: f64) {
        self.forward(-distance);
    }
}

#[cfg(test)]
mod tests {
    
}