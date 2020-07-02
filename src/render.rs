#[derive(Debug, Clone, Default)]
pub struct Point {
  pub x: f64,
  pub y: f64
}

pub struct Vec2 {
  pub x: f64,
  pub y: f64
}

impl Point {
  /// Create a new `Point`.
  fn new(x: f64, y: f64) -> Point {
    Point { x, y }
  }
}

pub trait RenderSurface {
  /// Draw a straight line between 2 points.
  fn draw_line(&mut self, p1: Point, p2: Point);
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

  pub fn forward(&mut self, distance: f64) {

  }

  pub fn backwards(&mut self, distance: f64) {
    
  }

  pub fn rotate(&mut self, angle: f64) {
    self.state.rotate(angle);
  }

  pub fn stack_push(&mut self) {
    let old_state = std::mem::replace(&mut self.state, TurtleState::new());
    self.stack.push(old_state);
  }

  pub fn stack_pop(&mut self) {
    self.state = self.stack.pop().expect("Cannot pop from an empty stack");
  }
}

#[derive(Debug, Clone, Default)]
pub struct TurtleState {
  position: Point,
  orientation: f64
}

impl TurtleState {
  /// Create a new `TurtleState`.
  pub fn new() -> TurtleState {
    TurtleState::default()
  }

  /// Return the current turtle direction vector.
  pub fn direction(&self) -> Vec2 {
    Vec2 { x: self.orientation.cos(), y: self.orientation.sin() }
  }

  /// Rotate the turtle by the given angle.
  pub fn rotate(&mut self, angle: f64) {
    self.orientation += angle;
  }
}